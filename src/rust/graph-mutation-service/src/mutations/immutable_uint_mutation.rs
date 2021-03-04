use crate::mutations::{UpsertGenerator, QueryInput};
use crate::v1beta1::ImmutableUintProp;
use crate::mutations::escape::{Escaped, escape_quote};
use crate::mutations::upsert_helpers::{gen_immutable_mutations, gen_immutable_query};

#[derive(Default)]
pub struct ImmutableUintUpsertGenerator {
    query_buffer: String,
    mutations: Vec<dgraph_tonic::Mutation>,
}

impl UpsertGenerator for ImmutableUintUpsertGenerator {
    type Input = ImmutableUintProp;
    fn generate_upserts(&mut self, creation_query: &QueryInput<'_>, predicate_name: &str, value: &Self::Input) -> (&str, &[dgraph_tonic::Mutation]) {
        let ImmutableUintProp {prop: ref value} = value;
        let value = Escaped::from(value);
        let query_suffix = format!("{}_{}_{}", &creation_query.unique_id, &creation_query.node_id, &creation_query.predicate_id);
        let query_name = gen_immutable_query(
            &mut self.query_buffer,
            &creation_query.creation_query_name,
            creation_query.node_id,
            &query_suffix,
            predicate_name,
        );

        gen_immutable_mutations(
            &mut self.mutations,
            &creation_query.creation_query_name,
            &query_name,
            &predicate_name,
            &value,
        );

        (&self.query_buffer, &self.mutations)
    }
}