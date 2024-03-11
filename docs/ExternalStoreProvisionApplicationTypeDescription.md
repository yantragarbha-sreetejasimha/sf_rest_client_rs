# ExternalStoreProvisionApplicationTypeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ProvisionApplicationTypeKind**](ProvisionApplicationTypeKind.md) | The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision. | [default to null]
**async** | **bool** | Indicates whether or not provisioning should occur asynchronously. When set to true, the provision operation returns when the request is accepted by the system, and the provision operation continues without any timeout limit. The default value is false. For large application packages, we recommend setting the value to true. | [default to null]
**application_package_download_uri** | **String** | The path to the &#39;.sfpkg&#39; application package from where the application package can be downloaded using HTTP or HTTPS protocols. The application package can be stored in an external store that provides GET operation to download the file. Supported protocols are HTTP and HTTPS, and the path must allow READ access. | [default to null]
**application_type_name** | **String** | The application type name represents the name of the application type found in the application manifest. | [default to null]
**application_type_version** | **String** | The application type version represents the version of the application type found in the application manifest. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


