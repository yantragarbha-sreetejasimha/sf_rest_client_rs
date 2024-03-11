# ManagedIdentityAzureBlobBackupStorageDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**storage_kind** | [***::models::BackupStorageKind**](BackupStorageKind.md) | The kind of backup storage, where backups are saved. | [default to null]
**friendly_name** | **String** | Friendly name for this backup storage. | [optional] [default to null]
**managed_identity_type** | [***::models::ManagedIdentityType**](ManagedIdentityType.md) | The type of managed identity to be used to connect to Azure Blob Store via Managed Identity. | [default to null]
**blob_service_uri** | **String** | The Blob Service Uri to connect to the Azure blob store.. | [default to null]
**container_name** | **String** | The name of the container in the blob store to store and enumerate backups from. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


