# VolumeProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | User readable description of the volume. | [optional] [default to null]
**status** | [***::models::ResourceStatus**](ResourceStatus.md) | Status of the volume. | [optional] [default to null]
**status_details** | **String** | Gives additional information about the current status of the volume. | [optional] [default to null]
**provider** | [***::models::VolumeProvider**](VolumeProvider.md) | Provider of the volume. | [default to null]
**azure_file_parameters** | [***::models::VolumeProviderParametersAzureFile**](VolumeProviderParametersAzureFile.md) | This type describes a volume provided by an Azure Files file share. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

