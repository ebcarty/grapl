# Implementing A Graph Model Plugin
Graph Model Plugins allow you to ‘Bring Your Own Model’ to Grapl. For example, if you wanted to implement a plugin for, say, AWS, which Grapl has no native support for, you would be adding an AWS Model to Grapl.

Models are split into a few components.

1. Python Schema Definitions - used for provisioning the GraphDB, among other things
2. Rust Schema Definitions - for graph generators to use
3. Analyzer Query and Views - used for detection and response

You only need to implement 1 and 2, the code for 3 will be generated for you.



## Rust Schema Definitions

In order to generate your graphs and implement a Graph Generator you’ll want to build a schema definition in rust, the language that we currently support for graph generation. As a reminder, graph generators are the services that turn raw data, like event logs, into a graph format that Grapl can understand.

You’ll need a relatively recent installation of rust, https://rustup.rs/

You can create a new rust library to define your schemas by running something like:


    cargo new grapl-aws-models

We can then add the necessary dependencies for Grapl:

    cargo add grapl-graph-descriptions
    cargo add derive-dynamic-node

Then, in your favorite IDE, navigate to the `src/lib.rs` file, where we’ll put our first model - the Ec2Instance.

`src/lib.rs`


    use derive_dynamic_node::{DynamicNode as DeriveDynamicNode, GraplStaticId};
    use grapl_graph_descriptions::graph_description::*;
    
    #[derive(Clone, DeriveDynamicNode, GraplStaticId)]
    struct Ec2Instance {
      #[static_id]
      arn: String,
      image_id: String,
      image_description: String,
      instance_id: String,
      launch_time: u64,
      instance_state: String,
      instance_type: String,
      availability_zone: String,
      platform: String,
    }
    
    impl IEc2InstanceNode for Ec2InstanceNode {
        fn get_mut_dynamic_node(&mut self) -> &mut DynamicNode {
            &mut self.dynamic_node
        }
    }

* Currently Grapl’s nodes must have only String, u64, or i64 properties.

The Ec2Instance struct is tagged with two important macros - DeriveDyanmicNode, and GraplStaticId.

The DeriveDynamicNode macro generates some code for us, in this case it will generate an `Ec2InstanceNode` structure, which is what we’ll store data in.

The GraplStaticId macro allows us to define a property, or properties, that can be used to identify the underlying entity. In AWS this is very straightforward - identity is provided by an Arn. Every node in Grapl must have an identity.

When parsing, we can add data to this node type like this:

    let mut ec2_instance = Ec2InstanceNode::new(
      Ec2InstanceNode::static_strategy()
    );
    
    ec2_instance.with_launch_time(launch_time);
    ec2_instance.with_instance_id(&instance_id);

The `Ec2InstanceNode` struct was generated by those macros, as was the method `static_strategy`, and the methods for adding data.


## Python Schema Definition

The Python schema definitions will serve two functions:

1. They will help us provision Grapl’s graph databases to understand our new model
2. They generate more Python code, which we’ll use in our Analyzers to detect and respond to threats using our new models


Our Python Schema for the Ec2InstanceNode will be relatively straightforward to implement.


    from grapl_analyzerlib.schemas.schema_builder import NodeSchema
    
    class Ec2InstanceNodeSchema(NodeSchema):
        def __init__(self):
            super(Ec2InstanceNodeSchema, self).__init__()
            (
                self
                .with_str_prop("arn")
                .with_str_prop("image_id")
                .with_str_prop("image_description")
                .with_str_prop("instance_id")
                .with_int_prop("launch_time")
                .with_str_prop("instance_state")
                .with_str_prop("instance_type")
                .with_str_prop("availability_zone")
                .with_str_prop("platform")
            )
            
        @staticmethod
        def self_type() -> str:
            return "Ec2Instance"

Make sure that the return value of the `self_type` method is the same name as the struct in your Rust model, in this case `Ec2Instance`.

Using this Ec2InstanceNodeSchema we can generate the rest of the code that we need for building signatures or responding to attacks.


    from grapl_analyzerlib.schemas.schema_builder import (
        generate_plugin_query, 
        generate_plugin_view
    )
    
    query = generate_plugin_query(Ec2InstanceNodeSchema())
    view = generate_plugin_view(Ec2InstanceNodeSchema())
    print(query)
    print(view)

This will generate and print out the code for querying or pivoting off of Ec2Instance nodes in Grapl.

Specifically it will generate the `Ec2InstanceQuery` and `Ec2InstanceView` classes.

You can just copy/paste this code into a file and load it up to use. There may be minor changes required, such as imports, but otherwise it should generally ‘just work’.


## Modifying the Graph Schema

Grapl already comes with the `Grapl Provision.ipynb` for provisioning the database. You can import our schemas into that database and then just add them to the schema list, which will be in a cell,


        schemas = (
            AssetSchema(),
            ProcessSchema(),
            FileSchema(),
            IpConnectionSchema(),
            IpAddressSchema(),
            IpPortSchema(),
            NetworkConnectionSchema(),
            ProcessInboundConnectionSchema(),
            ProcessOutboundConnectionSchema(),
            # Plugin Nodes
            Ec2InstanceNodeSchema(),
        )

Run the notebook and you should be good to go.



## Deploying Analyzers With Plugins

The simplest way to using Plugins in your Analyzers is to publish them to the PyPI and then add them as requirements to the `analyzer_executor/requirements.txt`, rebuild, and redeploy. At that point your analyzers can import the plugins and you can build out your graph signatures.