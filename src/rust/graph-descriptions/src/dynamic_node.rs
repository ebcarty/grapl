use dgraph_query_lib::mutation::{MutationPredicateValue,
                                 MutationUnit};
use log::warn;

use crate::{graph_description::{id_strategy,
                                node_property,
                                DynamicNode,
                                IdStrategy,
                                NodeProperty},
            node::NodeT};

impl DynamicNode {
    pub fn get_property(&self, name: impl AsRef<str>) -> Option<&NodeProperty> {
        self.properties.get(name.as_ref())
    }

    pub fn set_property(&mut self, name: impl Into<String>, value: impl Into<NodeProperty>) {
        self.properties.insert(name.into(), value.into().into());
    }

    pub fn set_key(&mut self, key: String) {
        self.node_key = key;
    }

    pub fn get_id_strategies(&self) -> &[IdStrategy] {
        &self.id_strategy[..]
    }

    pub fn requires_asset_identification(&self) -> bool {
        for strategy in self.get_id_strategies() {
            match strategy.strategy.as_ref().unwrap() {
                id_strategy::Strategy::Session(ref strategy) => {
                    if strategy.primary_key_requires_asset_id {
                        return true;
                    }
                }
                id_strategy::Strategy::Static(ref strategy) => {
                    if strategy.primary_key_requires_asset_id {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl NodeT for DynamicNode {
    fn get_asset_id(&self) -> Option<&str> {
        self.asset_id.as_ref().map(String::as_str)
    }

    fn set_asset_id(&mut self, asset_id: impl Into<String>) {
        self.asset_id = Some(asset_id.into());
    }

    fn get_node_key(&self) -> &str {
        self.node_key.as_str()
    }

    fn set_node_key(&mut self, node_key: impl Into<String>) {
        self.node_key = node_key.into();
    }

    fn merge(&mut self, other: &Self) -> bool {
        if self.node_key != other.node_key {
            warn!("Attempted to merge two NetworkConnection Nodes with differing node_keys");
            return false;
        }

        let mut merged = false;

        for (key, prop) in other.properties.clone() {
            let inserted = self.properties.insert(key, prop);
            if inserted.is_some() {
                merged = true;
            }
        }

        merged
    }

    fn merge_into(&mut self, other: Self) -> bool {
        if self.node_key != other.node_key {
            warn!("Attempted to merge two NetworkConnection Nodes with differing node_keys");
            return false;
        }

        let mut merged = false;

        for (key, prop) in other.properties.into_iter() {
            let inserted = self.properties.insert(key, prop);
            if inserted.is_some() {
                merged = true;
            }
        }

        merged
    }

    fn attach_predicates_to_mutation_unit(&self, mutation_unit: &mut MutationUnit) {
        mutation_unit.predicate_ref("node_key", MutationPredicateValue::string(&self.node_key));
        mutation_unit.predicate_ref(
            "seen_at",
            MutationPredicateValue::Number(self.seen_at as i64),
        );
        mutation_unit.predicate_ref(
            "dgraph.type",
            MutationPredicateValue::string(&self.node_type),
        );

        if let Some(asset_id) = &self.asset_id {
            mutation_unit.predicate_ref("asset_id", MutationPredicateValue::string(asset_id));
        }

        for (key, prop) in &self.properties {
            let prop = match &prop.property {
                Some(node_property::Property::Intprop(i)) => {
                    MutationPredicateValue::Number(*i as i64)
                }
                Some(node_property::Property::Uintprop(i)) => {
                    MutationPredicateValue::Number(*i as i64)
                }
                Some(node_property::Property::Strprop(s)) => MutationPredicateValue::string(s),
                None => panic!("Invalid property on DynamicNode: {}", self.node_key),
            };

            mutation_unit.predicate_ref(key, prop);
        }
    }

    fn get_cache_identities_for_predicates(&self) -> Vec<Vec<u8>> {
        let mut predicate_cache_identities = Vec::with_capacity(self.properties.len());

        for (key, prop) in &self.properties {
            let prop_value = match prop.property {
                Some(node_property::Property::Intprop(i)) => format!("{}", i),
                Some(node_property::Property::Uintprop(i)) => format!("{}", i),
                Some(node_property::Property::Strprop(ref s)) => s.clone(),
                None => panic!("Invalid property on DynamicNode: {}", self.node_key),
            };

            predicate_cache_identities.push(format!(
                "{}:{}:{}",
                self.get_node_key(),
                key,
                prop_value
            ));
        }

        predicate_cache_identities
            .into_iter()
            .map(|item| item.into_bytes())
            .collect()
    }
}
