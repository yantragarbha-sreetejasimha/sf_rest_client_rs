# UploadSessionInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**store_relative_path** | **String** | The remote location within image store. This path is relative to the image store root. | [optional] [default to null]
**session_id** | **String** | A unique ID of the upload session. A session ID can be reused only if the session was committed or removed. | [optional] [default to null]
**modified_date** | **String** | The date and time when the upload session was last modified. | [optional] [default to null]
**file_size** | **String** | The size in bytes of the uploading file. | [optional] [default to null]
**expected_ranges** | [**Vec<::models::UploadChunkRange>**](UploadChunkRange.md) | List of chunk ranges that image store has not received yet. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


