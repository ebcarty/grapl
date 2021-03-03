use std::convert::TryFrom;

use dgraph_query_lib::mutation::{MutationPredicateValue,
                                 MutationUnit};
use log::warn;
use uuid::Uuid;

use crate::{error::Error,
            graph_description::IpConnection,
            node::NodeT};

pub enum IpConnectionState {
    Created,
    Existing,
    Terminated,
}

impl From<IpConnectionState> for u32 {
    fn from(p: IpConnectionState) -> u32 {
        match p {
            IpConnectionState::Created => 1,
            IpConnectionState::Terminated => 2,
            IpConnectionState::Existing => 3,
        }
    }
}

impl TryFrom<u32> for IpConnectionState {
    type Error = Error;

    fn try_from(p: u32) -> Result<IpConnectionState, Error> {
        match p {
            1 => Ok(IpConnectionState::Created),
            2 => Ok(IpConnectionState::Terminated),
            3 => Ok(IpConnectionState::Existing),
            _ => Err(Error::InvalidIpConnectionState(p)),
        }
    }
}

impl IpConnection {
    pub fn new(
        src_ip_address: impl Into<String>,
        dst_ip_address: impl Into<String>,
        protocol: impl Into<String>,
        state: IpConnectionState,
        created_timestamp: u64,
        terminated_timestamp: u64,
        last_seen_timestamp: u64,
    ) -> Self {
        let src_ip_address = src_ip_address.into();
        let dst_ip_address = dst_ip_address.into();
        let protocol = protocol.into();

        Self {
            node_key: Uuid::new_v4().to_string(),
            src_ip_address,
            dst_ip_address,
            protocol,
            state: state.into(),
            created_timestamp,
            terminated_timestamp,
            last_seen_timestamp,
        }
    }
}

impl NodeT for IpConnection {
    fn get_asset_id(&self) -> Option<&str> {
        None
    }

    fn set_asset_id(&mut self, _asset_id: impl Into<String>) {
        panic!("Can not set asset_id on IpConnection");
    }

    fn get_node_key(&self) -> &str {
        &self.node_key
    }

    fn set_node_key(&mut self, node_key: impl Into<String>) {
        self.node_key = node_key.into();
    }

    fn merge(&mut self, other: &Self) -> bool {
        if self.node_key != other.node_key {
            warn!("Attempted to merge two IpConnection Nodes with differing node_keys");
            return false;
        }

        let mut merged = false;

        if self.created_timestamp == 0 || other.created_timestamp < self.created_timestamp {
            self.created_timestamp = other.created_timestamp;
            merged = true;
        }
        if self.terminated_timestamp == 0 || other.terminated_timestamp > self.terminated_timestamp
        {
            self.terminated_timestamp = other.terminated_timestamp;
            merged = true;
        }
        if self.last_seen_timestamp == 0 || other.last_seen_timestamp > self.last_seen_timestamp {
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
            MutationPredicateValue::string("IpConnection"),
        );
        mutation_unit.predicate_ref(
            "src_ip_address",
            MutationPredicateValue::string(&self.src_ip_address),
        );
        mutation_unit.predicate_ref(
            "dst_ip_address",
            MutationPredicateValue::string(&self.dst_ip_address),
        );
        mutation_unit.predicate_ref("protocol", MutationPredicateValue::string(&self.protocol));

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
            "src_ip_address",
            self.src_ip_address
        ));
        predicate_cache_identities.push(format!(
            "{}:{}:{}",
            self.get_node_key(),
            "dst_ip_address",
            self.dst_ip_address
        ));
        predicate_cache_identities.push(format!(
            "{}:{}:{}",
            self.get_node_key(),
            "protocol",
            self.protocol
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