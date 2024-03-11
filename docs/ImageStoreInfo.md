# ImageStoreInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disk_info** | [***::models::DiskInfo**](DiskInfo.md) | disk capacity and available disk space on the node where the ImageStore primary is placed. | [optional] [default to null]
**used_by_metadata** | [***::models::UsageInfo**](UsageInfo.md) | the ImageStore&#39;s file system usage for metadata. | [optional] [default to null]
**used_by_staging** | [***::models::UsageInfo**](UsageInfo.md) | The ImageStore&#39;s file system usage for staging files that are being uploaded. | [optional] [default to null]
**used_by_copy** | [***::models::UsageInfo**](UsageInfo.md) | the ImageStore&#39;s file system usage for copied application and cluster packages. [Removing application and cluster packages](https://docs.microsoft.com/en-us/rest/api/servicefabric/sfclient-api-deleteimagestorecontent) will free up this space. | [optional] [default to null]
**used_by_register** | [***::models::UsageInfo**](UsageInfo.md) | the ImageStore&#39;s file system usage for registered and cluster packages. [Unregistering application](https://docs.microsoft.com/en-us/rest/api/servicefabric/sfclient-api-unprovisionapplicationtype) and [cluster packages](https://docs.microsoft.com/en-us/rest/api/servicefabric/sfclient-api-unprovisionapplicationtype) will free up this space. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


