from typing import Optional

from infra import dynamodb
from infra.config import DEPLOYMENT_NAME, configurable_envvars
from infra.dgraph_cluster import DgraphCluster
from infra.dynamodb import DynamoDB
from infra.ec2 import Ec2Port
from infra.engagement_notebook import EngagementNotebook
from infra.lambda_ import Lambda, LambdaExecutionRole, LambdaResolver, PythonLambdaArgs
from infra.metric_forwarder import MetricForwarder
from infra.network import Network
from infra.secret import JWTSecret

import pulumi


# TODO: Rename to something like "Auth"
class EngagementEdge(pulumi.ComponentResource):
    def __init__(
        self,
        network: Network,
        secret: JWTSecret,
        db: DynamoDB,
        dgraph_cluster: DgraphCluster,
        forwarder: MetricForwarder,
        # This is optional ONLY because Localstack doesn't support
        # sagemaker :(
        notebook: Optional[EngagementNotebook] = None,
        opts: Optional[pulumi.ResourceOptions] = None,
    ) -> None:

        name = "engagement-edge"
        super().__init__("grapl:EngagementEdge", name, None, opts)

        self.role = LambdaExecutionRole(name, opts=pulumi.ResourceOptions(parent=self))

        self.function = Lambda(
            name,
            args=PythonLambdaArgs(
                handler="lambdex_handler.handler",
                code=LambdaResolver.resolve(name),
                env={
                    **configurable_envvars(name, ["GRAPL_LOG_LEVEL"]),
                    # TODO: Not clear that this is even used.
                    "MG_ALPHAS": dgraph_cluster.alpha_host_port,
                    "JWT_SECRET_ID": secret.secret.arn,
                    "USER_AUTH_TABLE": db.user_auth_table.name,
                    # TODO: This is a bit unfortunate... and only
                    # because Localstack doesn't support
                    # sagemaker. The alternative is to add additional
                    # "IS_LOCAL" logic to the engagement-edge service.
                    "GRAPL_NOTEBOOK_INSTANCE": notebook.name
                    if notebook
                    else f"{DEPLOYMENT_NAME}-Notebook",
                },
                timeout=25,
                memory_size=256,
                execution_role=self.role,
            ),
            network=network,
            opts=pulumi.ResourceOptions(parent=self),
        )

        Ec2Port("tcp", 443).allow_outbound_any_ip(self.function.security_group)

        forwarder.subscribe_to_log_group(name, self.function.log_group)

        # TODO: Original infrastructure code allowed access to DGraph,
        # but it's not clear this is even necessary.

        if notebook:
            notebook.grant_presigned_url_permissions_to(self.role)

        secret.grant_read_permissions_to(self.role)
        dynamodb.grant_read_on_tables(self.role, [db.user_auth_table])
        dgraph_cluster.allow_connections_from(self.function.security_group)

        self.register_outputs({})
