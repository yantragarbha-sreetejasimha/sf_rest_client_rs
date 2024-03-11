# CodePackageEntryPoint

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entry_point_location** | **String** | The location of entry point executable on the node. | [optional] [default to null]
**process_id** | **String** | The process ID of the entry point. | [optional] [default to null]
**run_as_user_name** | **String** | The user name under which entry point executable is run on the node. | [optional] [default to null]
**code_package_entry_point_statistics** | [***::models::CodePackageEntryPointStatistics**](CodePackageEntryPointStatistics.md) | Statistics about setup or main entry point  of a code package deployed on a Service Fabric node. | [optional] [default to null]
**status** | [***::models::EntryPointStatus**](EntryPointStatus.md) | Specifies the status of the code package entry point deployed on a Service Fabric node. | [optional] [default to null]
**next_activation_time** | **String** | The time (in UTC) when the entry point executable will be run next. | [optional] [default to null]
**instance_id** | [***::models::CodePackageInstanceId**](CodePackageInstanceId.md) | The instance ID for current running entry point. For a code package setup entry point (if specified) runs first and after it finishes main entry point is started. Each time entry point executable is run, its instance id will change. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


