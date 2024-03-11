# DiagnosticsDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sinks** | [**Vec<::models::DiagnosticsSinkProperties>**](DiagnosticsSinkProperties.md) | List of supported sinks that can be referenced. | [optional] [default to null]
**enabled** | **bool** | Status of whether or not sinks are enabled. | [optional] [default to null]
**default_sink_refs** | **Vec<String>** | The sinks to be used if diagnostics is enabled. Sink choices can be overridden at the service and code package level. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


