# \MeshGatewaysApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_gateway_create_or_update**](MeshGatewaysApi.md#mesh_gateway_create_or_update) | **Put** /Resources/Gateways/{gatewayResourceName} | Creates or updates a Gateway resource.
[**mesh_gateway_delete**](MeshGatewaysApi.md#mesh_gateway_delete) | **Delete** /Resources/Gateways/{gatewayResourceName} | Deletes the Gateway resource.
[**mesh_gateway_get**](MeshGatewaysApi.md#mesh_gateway_get) | **Get** /Resources/Gateways/{gatewayResourceName} | Gets the Gateway resource with the given name.
[**mesh_gateway_list**](MeshGatewaysApi.md#mesh_gateway_list) | **Get** /Resources/Gateways | Lists all the gateway resources.


# **mesh_gateway_create_or_update**
> ::models::GatewayResourceDescription mesh_gateway_create_or_update(api_version, gateway_resource_name, gateway_resource_description)
Creates or updates a Gateway resource.

Creates a Gateway resource with the specified name, description and properties. If Gateway resource with the same name exists, then it is updated with the specified description and properties. Use Gateway resource to provide public connectivity to application services.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **gateway_resource_name** | **String**| The identity of the gateway. | 
  **gateway_resource_description** | [**GatewayResourceDescription**](GatewayResourceDescription.md)| Description for creating a Gateway resource. | 

### Return type

[**::models::GatewayResourceDescription**](GatewayResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_gateway_delete**
> mesh_gateway_delete(api_version, gateway_resource_name)
Deletes the Gateway resource.

Deletes the Gateway resource identified by the name.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **gateway_resource_name** | **String**| The identity of the gateway. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_gateway_get**
> ::models::GatewayResourceDescription mesh_gateway_get(api_version, gateway_resource_name)
Gets the Gateway resource with the given name.

Gets the information about the Gateway resource with the given name. The information include the description and other properties of the Gateway.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **gateway_resource_name** | **String**| The identity of the gateway. | 

### Return type

[**::models::GatewayResourceDescription**](GatewayResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_gateway_list**
> ::models::PagedGatewayResourceDescriptionList mesh_gateway_list(api_version)
Lists all the gateway resources.

Gets the information about all gateway resources in a given resource group. The information include the description and other properties of the Gateway.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]

### Return type

[**::models::PagedGatewayResourceDescriptionList**](PagedGatewayResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

