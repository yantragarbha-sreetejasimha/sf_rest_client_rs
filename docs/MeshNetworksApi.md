# \MeshNetworksApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_network_create_or_update**](MeshNetworksApi.md#mesh_network_create_or_update) | **Put** /Resources/Networks/{networkResourceName} | Creates or updates a Network resource.
[**mesh_network_delete**](MeshNetworksApi.md#mesh_network_delete) | **Delete** /Resources/Networks/{networkResourceName} | Deletes the Network resource.
[**mesh_network_get**](MeshNetworksApi.md#mesh_network_get) | **Get** /Resources/Networks/{networkResourceName} | Gets the Network resource with the given name.
[**mesh_network_list**](MeshNetworksApi.md#mesh_network_list) | **Get** /Resources/Networks | Lists all the network resources.


# **mesh_network_create_or_update**
> ::models::NetworkResourceDescription mesh_network_create_or_update(api_version, network_resource_name, network_resource_description)
Creates or updates a Network resource.

Creates a Network resource with the specified name, description and properties. If Network resource with the same name exists, then it is updated with the specified description and properties. Network resource provides connectivity between application services.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **network_resource_name** | **String**| The identity of the network. | 
  **network_resource_description** | [**NetworkResourceDescription**](NetworkResourceDescription.md)| Description for creating a Network resource. | 

### Return type

[**::models::NetworkResourceDescription**](NetworkResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_network_delete**
> mesh_network_delete(api_version, network_resource_name)
Deletes the Network resource.

Deletes the Network resource identified by the name.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **network_resource_name** | **String**| The identity of the network. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_network_get**
> ::models::NetworkResourceDescription mesh_network_get(api_version, network_resource_name)
Gets the Network resource with the given name.

Gets the information about the Network resource with the given name. The information include the description and other properties of the Network.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **network_resource_name** | **String**| The identity of the network. | 

### Return type

[**::models::NetworkResourceDescription**](NetworkResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_network_list**
> ::models::PagedNetworkResourceDescriptionList mesh_network_list(api_version)
Lists all the network resources.

Gets the information about all network resources in a given resource group. The information include the description and other properties of the Network.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]

### Return type

[**::models::PagedNetworkResourceDescriptionList**](PagedNetworkResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

