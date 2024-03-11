# DeployedStatelessServiceInstanceDetailInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | Full hierarchical name of the service in URI format starting with &#x60;fabric:&#x60;. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [optional] [default to null]
**current_service_operation** | [***::models::ServiceOperationName**](ServiceOperationName.md) | Specifies the current active life-cycle operation on a stateful service replica or stateless service instance. | [optional] [default to null]
**current_service_operation_start_time_utc** | **String** | The start time of the current service operation in UTC format. | [optional] [default to null]
**reported_load** | [***::models::LoadMetricReportInfoList**](LoadMetricReportInfoList.md) | List of load reported by replica. | [optional] [default to null]
**instance_id** | [***::models::InstanceId**](InstanceId.md) | Id of a stateless service instance. InstanceId is used by Service Fabric to uniquely identify an instance of a partition of a stateless service. It is unique within a partition and does not change for the lifetime of the instance. If the instance has failed over on the same or different node, it will get a different value for the InstanceId. | [optional] [default to null]
**deployed_service_replica_query_result** | [***::models::DeployedStatelessServiceInstanceInfo**](DeployedStatelessServiceInstanceInfo.md) | Information about a stateless service instance deployed on a node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


