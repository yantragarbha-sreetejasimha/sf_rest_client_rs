# AzureInternalMonitoringPipelineSinkDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::DiagnosticsSinkKind**](DiagnosticsSinkKind.md) | The kind of DiagnosticsSink. | [default to null]
**name** | **String** | Name of the sink. This value is referenced by DiagnosticsReferenceDescription | [optional] [default to null]
**description** | **String** | A description of the sink. | [optional] [default to null]
**account_name** | **String** | Azure Internal monitoring pipeline account. | [optional] [default to null]
**namespace** | **String** | Azure Internal monitoring pipeline account namespace. | [optional] [default to null]
**ma_config_url** | **String** | Azure Internal monitoring agent configuration. | [optional] [default to null]
**fluentd_config_url** | [***Value**](Value.md) | Azure Internal monitoring agent fluentd configuration. | [optional] [default to null]
**auto_key_config_url** | **String** | Azure Internal monitoring pipeline autokey associated with the certificate. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


