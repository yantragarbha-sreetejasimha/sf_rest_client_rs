# StatefulServicePartitionInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**partition_status** | [***::models::ServicePartitionStatus**](ServicePartitionStatus.md) | The status of the service fabric service partition. | [optional] [default to null]
**partition_information** | [***::models::PartitionInformation**](PartitionInformation.md) | Information about the partition identity, partitioning scheme and keys supported by it. | [optional] [default to null]
**target_replica_set_size** | **i64** | The target replica set size as a number. | [optional] [default to null]
**min_replica_set_size** | **i64** | The minimum replica set size as a number. | [optional] [default to null]
**last_quorum_loss_duration** | **String** | The duration for which this partition was in quorum loss. If the partition is currently in quorum loss, it returns the duration since it has been in that state. This field is using ISO8601 format for specifying the duration. | [optional] [default to null]
**primary_epoch** | [***::models::Epoch**](Epoch.md) | An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


