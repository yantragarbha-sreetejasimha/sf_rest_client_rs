# SecretResourceProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::SecretKind**](SecretKind.md) | Describes the kind of secret. | [default to null]
**description** | **String** | User readable description of the secret. | [optional] [default to null]
**status** | [***::models::ResourceStatus**](ResourceStatus.md) | Status of the resource. | [optional] [default to null]
**status_details** | **String** | Gives additional information about the current status of the secret. | [optional] [default to null]
**content_type** | **String** | The type of the content stored in the secret value. The value of this property is opaque to Service Fabric. Once set, the value of this property cannot be changed. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


