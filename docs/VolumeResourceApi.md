# \VolumeResourceApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume_resource**](VolumeResourceApi.md#create_volume_resource) | **Put** /Resources/Volumes/{volumeResourceName} | Creates or updates a volume resource.
[**delete_volume_resource**](VolumeResourceApi.md#delete_volume_resource) | **Delete** /Resources/Volumes/{volumeResourceName} | Deletes the volume resource.
[**get_volume_resource**](VolumeResourceApi.md#get_volume_resource) | **Get** /Resources/Volumes/{volumeResourceName} | Gets the volume resource.


# **create_volume_resource**
> create_volume_resource(api_version, volume_resource_name, volume_resource_description)
Creates or updates a volume resource.

Creates a volume resource with the specified name and description. If a volume with the same name already exists, then its description is updated to the one indicated in this request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **volume_resource_name** | **String**| Service Fabric volume resource name. | 
  **volume_resource_description** | [**VolumeResourceDescription**](VolumeResourceDescription.md)| Description for creating a volume resource. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_volume_resource**
> delete_volume_resource(api_version, volume_resource_name)
Deletes the volume resource.

Deletes the volume identified by the name.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **volume_resource_name** | **String**| Service Fabric volume resource name. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_volume_resource**
> ::models::VolumeResourceDescription get_volume_resource(api_version, volume_resource_name)
Gets the volume resource.

Gets the information about the volume resource with a given name. This information includes the volume description and other runtime information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **volume_resource_name** | **String**| Service Fabric volume resource name. | 

### Return type

[**::models::VolumeResourceDescription**](VolumeResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

