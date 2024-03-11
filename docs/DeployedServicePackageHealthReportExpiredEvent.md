# DeployedServicePackageHealthReportExpiredEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | [***::models::ApplicationId**](ApplicationId.md) | The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \&quot;\\~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp\\~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | [optional] [default to null]
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**category** | **String** | The category of event. | [optional] [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**service_manifest** | **String** | Service manifest name. | [default to null]
**service_package_instance_id** | **i64** | Id of Service package instance. | [default to null]
**service_package_activation_id** | **String** | Id of Service package activation. | [default to null]
**node_name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [default to null]
**source_id** | **String** | Id of report source. | [default to null]
**property** | **String** | Describes the property. | [default to null]
**health_state** | **String** | Describes the property health state. | [default to null]
**time_to_live_ms** | **i64** | Time to live in milli-seconds. | [default to null]
**sequence_number** | **i64** | Sequence number of report. | [default to null]
**description** | **String** | Description of report. | [default to null]
**remove_when_expired** | **bool** | Indicates the removal when it expires. | [default to null]
**source_utc_timestamp** | **String** | Source time. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


