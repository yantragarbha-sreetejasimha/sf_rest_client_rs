# UnprovisionApplicationTypeDescriptionInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_type_version** | [***::models::ApplicationTypeVersion**](ApplicationTypeVersion.md) | The version of the application type as defined in the application manifest. | [default to null]
**async** | **bool** | The flag indicating whether or not unprovision should occur asynchronously. When set to true, the unprovision operation returns when the request is accepted by the system, and the unprovision operation continues without any timeout limit. The default value is false. However, we recommend setting it to true for large application packages that were provisioned. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


