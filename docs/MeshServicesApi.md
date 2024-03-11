# \MeshServicesApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_service_get**](MeshServicesApi.md#mesh_service_get) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName} | Gets the Service resource with the given name.
[**mesh_service_list**](MeshServicesApi.md#mesh_service_list) | **Get** /Resources/Applications/{applicationResourceName}/Services | Lists all the service resources.


# **mesh_service_get**
> ::models::ServiceResourceDescription mesh_service_get(api_version, application_resource_name, service_resource_name)
Gets the Service resource with the given name.

Gets the information about the Service resource with the given name. The information include the description and other properties of the Service.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 
  **service_resource_name** | **String**| The identity of the service. | 

### Return type

[**::models::ServiceResourceDescription**](ServiceResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_service_list**
> ::models::PagedServiceResourceDescriptionList mesh_service_list(api_version, application_resource_name)
Lists all the service resources.

Gets the information about all services of an application resource. The information include the description and other properties of the Service.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 

### Return type

[**::models::PagedServiceResourceDescriptionList**](PagedServiceResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

