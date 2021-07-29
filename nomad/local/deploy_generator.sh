#!/bin/bash

# Creates a namespace in nomad and deploys a generator into it

read -p "Tenant ID: " tenant_id
read -p "Generator Ingest: " generator_ingest
read -p "Artifact URL: " artifact_url
read -p "Instances: " generator_count

nomad namespace apply -description "Tenant ${tenant_id}'s ${generator_ingest} generator." "generator-${tenant_id}-${generator_ingest}"

nomad job run \
    -var "tenant_id=${tenant_id}" \
    -var "generator_count=${generator_count}" \
    -var "generator_ingest=${generator_ingest}" \
    -var "generator_artifact_url=${artifact_url}" \
    grapl-generator.nomad