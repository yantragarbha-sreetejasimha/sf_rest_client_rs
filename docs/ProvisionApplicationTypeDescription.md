# ProvisionApplicationTypeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ProvisionApplicationTypeKind**](ProvisionApplicationTypeKind.md) | The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision. | [default to null]
**async** | **bool** | Indicates whether or not provisioning should occur asynchronously. When set to true, the provision operation returns when the request is accepted by the system, and the provision operation continues without any timeout limit. The default value is false. For large application packages, we recommend setting the value to true. | [default to null]
**application_type_build_path** | **String** | The relative path for the application package in the image store specified during the prior upload operation. | [default to null]
**application_package_cleanup_policy** | [***::models::ApplicationPackageCleanupPolicy**](ApplicationPackageCleanupPolicy.md) | The kind of action that needs to be taken for cleaning up the application package after successful provision. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


