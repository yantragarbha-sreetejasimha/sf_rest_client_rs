# ServiceDeletedEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**service_id** | [***::models::ServiceId**](ServiceId.md) | The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource. Starting in version 6.0, hierarchical names are delimited with the \&quot;\\~\&quot; character. For example, if the service name is \&quot;fabric:/myapp/app1/svc1\&quot;, the service identity would be \&quot;myapp~app1\\~svc1\&quot; in 6.0+ and \&quot;myapp/app1/svc1\&quot; in previous versions. | [default to null]
**service_type_name** | **String** | Service type name. | [default to null]
**application_name** | **String** | Application name. | [default to null]
**application_type_name** | **String** | Application type name. | [default to null]
**service_instance** | **i64** | Id of Service instance. | [default to null]
**is_stateful** | **bool** | Indicates if Service is stateful. | [default to null]
**partition_count** | **i32** | Number of partitions. | [default to null]
**target_replica_set_size** | **i32** | Size of target replicas set. | [default to null]
**min_replica_set_size** | **i32** | Minimum size of replicas set. | [default to null]
**service_package_version** | **String** | Version of Service package. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


