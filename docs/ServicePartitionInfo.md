# ServicePartitionInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**partition_status** | [***::models::ServicePartitionStatus**](ServicePartitionStatus.md) | The status of the service fabric service partition. | [optional] [default to null]
**partition_information** | [***::models::PartitionInformation**](PartitionInformation.md) | Information about the partition identity, partitioning scheme and keys supported by it. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


