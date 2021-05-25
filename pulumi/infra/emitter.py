import json
from typing import Optional

import pulumi_aws as aws
from infra.bucket import Bucket
from infra.config import DEPLOYMENT_NAME

import pulumi


class EventEmitter(pulumi.ComponentResource):
    """
    Buckets that send events to SNS topics.
    """

    def __init__(
        self, event_name: str, opts: Optional[pulumi.ResourceOptions] = None
    ) -> None:

        super().__init__("grapl:EventEmitter", event_name, None, opts)

        logical_bucket_name = f"{event_name}-bucket"
        self.bucket = Bucket(
            logical_bucket_name, sse=True, opts=pulumi.ResourceOptions(parent=self)
        )

        physical_topic_name = f"{DEPLOYMENT_NAME}-{event_name}-topic"
        self.topic = aws.sns.Topic(
            f"{event_name}-topic",
            name=physical_topic_name,
            opts=pulumi.ResourceOptions(parent=self),
        )

        # This is a resource-based policy to allow our bucket to
        # publish to our topic, which in turn allows us to set up the
        # bucket notification below.
        self.topic_policy_attachment = aws.sns.TopicPolicy(
            f"{event_name}-bucket-publishes-to-topic",
            arn=self.topic.arn,
            policy=pulumi.Output.all(self.topic.arn, self.bucket.arn).apply(
                lambda topic_and_bucket: json.dumps(
                    {
                        "Version": "2012-10-17",
                        "Statement": [
                            {
                                "Sid": "0",
                                "Effect": "Allow",
                                "Principal": {
                                    "Service": "s3.amazonaws.com",
                                },
                                "Action": "sns:Publish",
                                "Resource": topic_and_bucket[0],
                                "Condition": {
                                    "ArnLike": {"aws:SourceArn": topic_and_bucket[1]}
                                },
                            }
                        ],
                    }
                )
            ),
            opts=pulumi.ResourceOptions(parent=self),
        )

        self.bucket_notification = aws.s3.BucketNotification(
            f"{logical_bucket_name}-notifies-topic",
            bucket=self.bucket.id,
            topics=[
                aws.s3.BucketNotificationTopicArgs(
                    topic_arn=self.topic.arn,
                    events=["s3:ObjectCreated:*"],
                )
            ],
            opts=pulumi.ResourceOptions(
                parent=self,
                depends_on=[self.topic_policy_attachment],
            ),
        )

        self.register_outputs({})

    def grant_write_to(self, role: aws.iam.Role) -> None:
        aws.iam.RolePolicy(
            f"{role._name}-writes-objects-to-{self.bucket._name}",
            role=role.name,
            policy=self.bucket.arn.apply(
                lambda bucket_arn: json.dumps(
                    {
                        "Version": "2012-10-17",
                        "Statement": [
                            {
                                "Effect": "Allow",
                                "Action": [
                                    "s3:Abort*",
                                    "s3:DeleteObject*",
                                    "s3:PutObject*",
                                ],
                                "Resource": [bucket_arn, f"{bucket_arn}/*"],
                            }
                        ],
                    }
                )
            ),
            opts=pulumi.ResourceOptions(parent=role),
        )

    def grant_read_to(self, role: aws.iam.Role) -> None:
        aws.iam.RolePolicy(
            f"{role._name}-reads-objects-from-{self.bucket._name}",
            role=role.name,
            policy=self.bucket.arn.apply(
                lambda bucket_arn: json.dumps(
                    {
                        "Version": "2012-10-17",
                        "Statement": [
                            {
                                "Effect": "Allow",
                                "Action": "s3:GetObject",
                                "Resource": f"{bucket_arn}/*",
                            }
                        ],
                    }
                )
            ),
            opts=pulumi.ResourceOptions(parent=role),
        )
