# StatelessServicePartitionInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**partition_status** | [***::models::ServicePartitionStatus**](ServicePartitionStatus.md) | The status of the service fabric service partition. | [optional] [default to null]
**partition_information** | [***::models::PartitionInformation**](PartitionInformation.md) | Information about the partition identity, partitioning scheme and keys supported by it. | [optional] [default to null]
**instance_count** | **i64** | Number of instances of this partition. | [optional] [default to null]
**min_instance_count** | [***::models::MinInstanceCount**](MinInstanceCount.md) | MinInstanceCount is the minimum number of instances that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node. The actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ). Note, if InstanceCount is set to -1, during MinInstanceCount computation -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service. | [optional] [default to null]
**min_instance_percentage** | [***::models::MinInstancePercentage**](MinInstancePercentage.md) | MinInstancePercentage is the minimum percentage of InstanceCount that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node. The actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ). Note, if InstanceCount is set to -1, during MinInstancePercentage computation, -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


