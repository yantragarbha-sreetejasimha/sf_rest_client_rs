# \MeshVolumesApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_volume_create_or_update**](MeshVolumesApi.md#mesh_volume_create_or_update) | **Put** /Resources/Volumes/{volumeResourceName} | Creates or updates a Volume resource.
[**mesh_volume_delete**](MeshVolumesApi.md#mesh_volume_delete) | **Delete** /Resources/Volumes/{volumeResourceName} | Deletes the Volume resource.
[**mesh_volume_get**](MeshVolumesApi.md#mesh_volume_get) | **Get** /Resources/Volumes/{volumeResourceName} | Gets the Volume resource with the given name.
[**mesh_volume_list**](MeshVolumesApi.md#mesh_volume_list) | **Get** /Resources/Volumes | Lists all the volume resources.


# **mesh_volume_create_or_update**
> ::models::VolumeResourceDescription mesh_volume_create_or_update(api_version, volume_resource_name, volume_resource_description)
Creates or updates a Volume resource.

Creates a Volume resource with the specified name, description and properties. If Volume resource with the same name exists, then it is updated with the specified description and properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **volume_resource_name** | **String**| The identity of the volume. | 
  **volume_resource_description** | [**VolumeResourceDescription**](VolumeResourceDescription.md)| Description for creating a Volume resource. | 

### Return type

[**::models::VolumeResourceDescription**](VolumeResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_volume_delete**
> mesh_volume_delete(api_version, volume_resource_name)
Deletes the Volume resource.

Deletes the Volume resource identified by the name.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **volume_resource_name** | **String**| The identity of the volume. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_volume_get**
> ::models::VolumeResourceDescription mesh_volume_get(api_version, volume_resource_name)
Gets the Volume resource with the given name.

Gets the information about the Volume resource with the given name. The information include the description and other properties of the Volume.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **volume_resource_name** | **String**| The identity of the volume. | 

### Return type

[**::models::VolumeResourceDescription**](VolumeResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_volume_list**
> ::models::PagedVolumeResourceDescriptionList mesh_volume_list(api_version)
Lists all the volume resources.

Gets the information about all volume resources in a given resource group. The information include the description and other properties of the Volume.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]

### Return type

[**::models::PagedVolumeResourceDescriptionList**](PagedVolumeResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

