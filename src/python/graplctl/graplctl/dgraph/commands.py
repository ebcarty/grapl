import click
import graplctl.dgraph.lib as dgraph_ops
import graplctl.swarm.lib as docker_swarm_ops
from graplctl.common import State, pass_graplctl_state
from mypy_boto3_ec2.literals import InstanceTypeType

#
# dgraph operational commands
#


@click.group()
@click.pass_context
@pass_graplctl_state
def dgraph(
    graplctl_state: State,
    ctx: click.Context,
) -> None:
    """commands for operating dgraph"""
    pass


@dgraph.command()
@click.option(
    "-t",
    "--instance-type",
    type=click.Choice(choices=("i3.large", "i3.xlarge", "i3.2xlarge")),
    help="EC2 instance type for swarm nodes",
    required=True,
)
@click.option(
    "-b",
    "--dgraph-config-bucket",
    help="Name of the S3 bucket with Dgraph config files for the cluster",
    type=click.STRING,
    required=True,
    envvar="GRAPL_DGRAPH_CONFIG_BUCKET",
)
@click.option(
    "-l",
    "--dgraph-logs-group",
    help="The name of the AWS logs group to which the containers of the Dgraph Swarm cluster will send their log streams",
    type=click.STRING,
    required=True,
    envvar="GRAPL_DGRAPH_LOGS_GROUP",
)
@click.option(
    "-s",
    "--swarm-config-bucket",
    type=click.STRING,
    help="Name of the S3 bucket with Swarm config files for the cluster",
    required=True,
    envvar="GRAPL_SWARM_CONFIG_BUCKET",
)
@pass_graplctl_state
def create(
    graplctl_state: State,
    instance_type: InstanceTypeType,
    dgraph_config_bucket: str,
    dgraph_logs_group: str,
    swarm_config_bucket: str,
) -> None:
    """spin up a swarm cluster and deploy dgraph on it"""
    # TODO: Make idempotent
    click.echo(f"creating dgraph cluster of {instance_type} instances")
    if not dgraph_ops.create_dgraph(
        graplctl_state=graplctl_state,
        instance_type=instance_type,
        dgraph_config_bucket=dgraph_config_bucket,
        dgraph_logs_group=dgraph_logs_group,
        swarm_config_bucket=swarm_config_bucket,
    ):
        click.echo("dgraph cluster already exists")
        return
    click.echo(f"created dgraph cluster of {instance_type} instances")
    click.echo(f"(Don't forget to `graplctl aws provision` next!)")


@dgraph.command()
@click.option(
    "-i",
    "--swarm-id",
    type=click.STRING,
    help="unique id of the swarm cluster",
    required=True,
)
@click.confirmation_option(
    prompt="are you sure you want to remove the dgraph dns records?"
)
@pass_graplctl_state
def remove_dns(graplctl_state: State, swarm_id: str) -> None:
    """remove dgraph dns records"""
    click.echo(f"removing dgraph dns records for swarm {swarm_id}")
    dgraph_ops.remove_dgraph_dns(graplctl_state=graplctl_state, swarm_id=swarm_id)
    click.echo(f"removed dgraph dns records for swarm {swarm_id}")


@dgraph.command()
@click.confirmation_option(prompt="are you quite sure?")
@pass_graplctl_state
def destroy(graplctl_state: State) -> None:
    """nuke all the swarm clusters from orbit"""
    click.echo("destroying swarm clusters")
    for swarm_id in docker_swarm_ops.swarm_ids(
        ec2=graplctl_state.ec2,
        deployment_name=graplctl_state.grapl_deployment_name,
        version=graplctl_state.grapl_version,
        region=graplctl_state.grapl_region,
    ):
        click.echo(f"removing swarm dns records for swarm {swarm_id}")
        dgraph_ops.remove_dgraph_dns(graplctl_state=graplctl_state, swarm_id=swarm_id)
        click.echo(f"removed swarm dns records for swarm {swarm_id}")

        click.echo(f"destroying swarm cluster {swarm_id}")
        docker_swarm_ops.destroy_swarm(graplctl_state=graplctl_state, swarm_id=swarm_id)
        click.echo(f"destroyed swarm cluster {swarm_id}")
    click.echo("destroyed swarm cluster")
