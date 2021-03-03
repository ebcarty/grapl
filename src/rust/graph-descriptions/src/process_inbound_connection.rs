use std::convert::TryFrom;

use dgraph_query_lib::mutation::{MutationPredicateValue,
                                 MutationUnit};
use log::warn;
use uuid::Uuid;

use crate::{error::Error,
            graph_description::ProcessInboundConnection,
            node::NodeT};

pub enum ProcessInboundConnectionState {
    Bound,
    Existing,
    Closed,
}

impl From<ProcessInboundConnectionState> for u32 {
    fn from(p: ProcessInboundConnectionState) -> u32 {
        match p {
            ProcessInboundConnectionState::Bound => 1,
            ProcessInboundConnectionState::Closed => 2,
            ProcessInboundConnectionState::Existing => 3,
        }
    }
}

impl TryFrom<u32> for ProcessInboundConnectionState {
    type Error = Error;

    fn try_from(p: u32) -> Result<ProcessInboundConnectionState, Error> {
        match p {
            1 => Ok(ProcessInboundConnectionState::Bound),
            2 => Ok(ProcessInboundConnectionState::Closed),
            3 => Ok(ProcessInboundConnectionState::Existing),
            _ => Err(Error::InvalidProcessInboundConnectionState(p)),
        }
    }
}

impl ProcessInboundConnection {
    pub fn new(
        asset_id: impl Into<Option<String>>,
        hostname: impl Into<Option<String>>,
        state: ProcessInboundConnectionState,
        port: u16,
        ip_address: impl Into<String>,
        protocol: impl Into<String>,
        created_timestamp: u64,
        terminated_timestamp: u64,
        last_seen_timestamp: u64,
    ) -> Self {
        let asset_id = asset_id.into();
        let hostname = hostname.into();
        let protocol = protocol.into();

        if hostname.is_none() && asset_id.is_none() {
            panic!("ProcessInboundConnection must have at least asset_id or hostname");
        }

        let ip_address = ip_address.into();

        Self {
            node_key: Uuid::new_v4().to_string(),
            ip_address,
            asset_id,
            hostname,
            protocol,
            created_timestamp,
            terminated_timestamp,
            last_seen_timestamp,
            port: port as u32,
            state: state.into(),
        }
    }
}

impl NodeT for ProcessInboundConnection {
    fn get_asset_id(&self) -> Option<&str> {
        self.asset_id.as_ref().map(String::as_str)
    }

    fn set_asset_id(&mut self, asset_id: impl Into<String>) {
        self.asset_id = Some(asset_id.into());
    }

    fn get_node_key(&self) -> &str {
        &self.node_key
    }

    fn set_node_key(&mut self, node_key: impl Into<String>) {
        self.node_key = node_key.into();
    }

    fn merge(&mut self, other: &Self) -> bool {
        if self.node_key != other.node_key {
            warn!("Attempted to merge two ProcessInboundConnection Nodes with differing node_keys");
            return false;
        }

        if self.ip_address != other.ip_address {
            warn!("Attempted to merge two ProcessInboundConnection Nodes with differing IPs");
            return false;
        }

        let mut merged = false;

        if self.asset_id.is_none() && other.asset_id.is_some() {
            self.asset_id = other.asset_id.clone();
        }

        if self.hostname.is_none() && other.hostname.is_some() {
            self.hostname = other.hostname.clone();
        }

        if self.created_timestamp != 0 && self.created_timestamp > other.created_timestamp {
            self.created_timestamp = other.created_timestamp;
            merged = true;
        }

        if self.terminated_timestamp != 0 && self.terminated_timestamp < other.terminated_timestamp
        {
            self.terminated_timestamp = other.terminated_timestamp;
            merged = true;
        }

        if self.last_seen_timestamp != 0 && self.last_seen_timestamp < other.last_seen_timestamp {
            self.last_seen_timestamp = other.last_seen_timestamp;
            merged = true;
        }

        merged
    }

    fn merge_into(&mut self, other: Self) -> bool {
        self.merge(&other)
    }

    fn attach_predicates_to_mutation_unit(&self, mutation_unit: &mut MutationUnit) {
        mutation_unit.predicate_ref("node_key", MutationPredicateValue::string(&self.node_key));
        mutation_unit.predicate_ref(
            "dgraph.type",
            MutationPredicateValue::string("ProcessInboundConnection"),
        );
        mutation_unit.predicate_ref("protocol", MutationPredicateValue::string(&self.protocol));
        mutation_unit.predicate_ref("port", MutationPredicateValue::Number(self.port as i64));

        if self.created_timestamp != 0 {
            mutation_unit.predicate_ref(
                "created_timestamp",
                MutationPredicateValue::Number(self.created_timestamp as i64),
            );
        }

        if self.terminated_timestamp != 0 {
            mutation_unit.predicate_ref(
                "terminated_timestamp",
                MutationPredicateValue::Number(self.terminated_timestamp as i64),
            );
        }

        if self.last_seen_timestamp != 0 {
            mutation_unit.predicate_ref(
                "last_seen_timestamp",
                MutationPredicateValue::Number(self.last_seen_timestamp as i64),
            );
        }
    }

    fn get_cache_identities_for_predicates(&self) -> Vec<Vec<u8>> {
        let mut predicate_cache_identities = Vec::new();

        predicate_cache_identities.push(format!(
            "{}:{}:{}",
            self.get_node_key(),
            "protocol",
            self.protocol
        ));
        predicate_cache_identities.push(format!(
            "{}:{}:{}",
            self.get_node_key(),
            "port",
            self.port
        ));

        if self.created_timestamp != 0 {
            predicate_cache_identities.push(format!(
                "{}:{}:{}",
                self.get_node_key(),
                "created_timestamp",
                self.created_timestamp
            ));
        }
        if self.terminated_timestamp != 0 {
            predicate_cache_identities.push(format!(
                "{}:{}:{}",
                self.get_node_key(),
                "terminated_timestamp",
                self.terminated_timestamp
            ));
        }
        if self.last_seen_timestamp != 0 {
            predicate_cache_identities.push(format!(
                "{}:{}:{}",
                self.get_node_key(),
                "last_seen_timestamp",
                self.last_seen_timestamp
            ));
        }

        predicate_cache_identities
            .into_iter()
            .map(|item| item.into_bytes())
            .collect()
    }
}
