# ContainerDeactivatedEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**application_id** | [***::models::ApplicationId**](ApplicationId.md) | The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \&quot;\\~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp\\~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | [default to null]
**service_name** | **String** | Name of Service. | [default to null]
**service_package_name** | **String** | Name of Service package. | [default to null]
**service_package_activation_id** | **String** | Activation Id of Service package. | [default to null]
**is_exclusive** | **bool** | Indicates IsExclusive flag. | [default to null]
**code_package_name** | **String** | Name of Code package. | [default to null]
**entry_point_type** | **String** | Type of EntryPoint. | [default to null]
**image_name** | **String** | Name of Container image. | [default to null]
**container_name** | **String** | Name of Container. | [default to null]
**host_id** | **String** | Host Id. | [default to null]
**exit_code** | **i64** | Exit code of process. | [default to null]
**unexpected_termination** | **bool** | Indicates if termination is unexpected. | [default to null]
**start_time** | **String** | Start time of process. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


