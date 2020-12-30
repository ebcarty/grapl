use crate::metrics::SysmonSubgraphGeneratorMetrics;
use crate::models::SysmonTryFrom;
use async_trait::async_trait;
use failure::bail;
use grapl_graph_descriptions::graph_description::*;
use grapl_observe::log_time;
use log::*;
use sqs_executor::cache::{Cache, CacheResponse};
use sqs_executor::errors::{CheckedError, Recoverable};
use sqs_executor::event_handler::{CompletedEvents, EventHandler};
use std::borrow::Cow;
use sysmon::Event;

#[derive(thiserror::Error, Debug)]
pub enum SysmonGeneratorError {
    #[error("Generator failed")]
    E(failure::Error),
    #[error("DeserializeError")]
    DeserializeError(failure::Error),
    #[error("Generator failed")]
    Unexpected,
}

impl CheckedError for SysmonGeneratorError {
    fn error_type(&self) -> Recoverable {
        match self {
            Self::DeserializeError(_) => Recoverable::Persistent,
            Self::E(_) => Recoverable::Persistent,
            Self::Unexpected => Recoverable::Transient,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SysmonSubgraphGenerator<C>
where
    C: Cache + Clone + Send + Sync + 'static,
{
    cache: C,
    metrics: SysmonSubgraphGeneratorMetrics,
}

impl<C> SysmonSubgraphGenerator<C>
where
    C: Cache + Clone + Send + Sync + 'static,
{
    pub fn new(cache: C, metrics: SysmonSubgraphGeneratorMetrics) -> Self {
        Self { cache, metrics }
    }

    /// Takes a vec of event Strings, parses them, and converts them into subgraphs
    async fn process_events(
        &mut self,
        events: Vec<Cow<'_, str>>,
    ) -> (Graph, Vec<Event>, Option<SysmonGeneratorError>) {
        let mut last_failure: Option<SysmonGeneratorError> = None;
        let mut identities = Vec::with_capacity(events.len());
        let mut final_subgraph = Graph::new(0);

        for event in events {
            let event = match Event::from_str(&event) {
                Ok(event) => event,
                Err(e) => {
                    warn!("Failed to deserialize event: {}, {}", e, event);

                    last_failure = Some(SysmonGeneratorError::DeserializeError(failure::err_msg(
                        format!("Failed: {}", e),
                    )));

                    continue;
                }
            };

            match self.cache.get(event.clone()).await {
                Ok(CacheResponse::Hit) => {
                    info!("Got cached response");
                    continue;
                }
                Err(e) => warn!("Cache failed with: {:?}", e),
                _ => (),
            };

            let graph = match Graph::try_from(event.clone()) {
                Ok(subgraph) => subgraph,
                Err(e) => {
                    // TODO: we should probably be recording each separate failure, but this is only going to save the last failure
                    last_failure = Some(SysmonGeneratorError::E(e));
                    continue;
                }
            };

            identities.push(event);

            final_subgraph.merge(&graph);
        }

        (final_subgraph, identities, last_failure)
    }
}

#[async_trait]
impl<C> EventHandler for SysmonSubgraphGenerator<C>
where
    C: Cache + Clone + Send + Sync + 'static,
{
    type InputEvent = Vec<u8>;
    type OutputEvent = Graph;
    type Error = SysmonGeneratorError;

    async fn handle_event(
        &mut self,
        events: Vec<u8>,
        completed: &mut CompletedEvents,
    ) -> Result<Self::OutputEvent, Result<(Self::OutputEvent, Self::Error), Self::Error>> {
        info!("Handling raw event");

        /*
           This iterator is taking a set of bytes of the logs, splitting the logs on newlines,
           converting the byte sequences to utf-8 strings, and then filtering on the following criteria:
               1. The line isn't empty
               2. The line is not `\n` (to prevent issues with multiple newline sequences)
               3. The line contains event with ID 1, 3, or 11

           The event ids 1, 3, and 11 correspond to Process Creation, Network Connection, and File Creation
           in that order.

           https://docs.microsoft.com/en-us/sysinternals/downloads/sysmon#events
        */
        let events: Vec<_> = log_time!(
            "event split",
            events
                .split(|i| &[*i][..] == &b"\n"[..])
                .map(String::from_utf8_lossy)
                .filter(|event| {
                    (!event.is_empty() && event != "\n")
                        && (event.contains(&"EventID>1<"[..])
                            || event.contains(&"EventID>3<"[..])
                            || event.contains(&"EventID>11<"[..]))
                })
                .collect()
        );

        info!("Handling {} events", events.len());

        let (final_subgraph, identities, last_failure) = self.process_events(events).await;

        info!("Completed mapping {} subgraphs", identities.len());
        self.metrics
            .report_handle_event_success(last_failure.is_some());

        identities
            .into_iter()
            .for_each(|identity| completed.add_identity(identity));

        match last_failure {
            Some(e) => Err(Ok((final_subgraph, e))),
            None => Ok(final_subgraph),
        }
    }
}
