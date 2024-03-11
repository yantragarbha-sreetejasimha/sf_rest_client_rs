# ImageStoreCopyDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**remote_source** | **String** | The relative path of source image store content to be copied from. | [default to null]
**remote_destination** | **String** | The relative path of destination image store content to be copied to. | [default to null]
**skip_files** | **Vec<String>** | The list of the file names to be skipped for copying. | [optional] [default to null]
**check_mark_file** | **bool** | Indicates whether to check mark file during copying. The property is true if checking mark file is required, false otherwise. The mark file is used to check whether the folder is well constructed. If the property is true and mark file does not exist, the copy is skipped. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


