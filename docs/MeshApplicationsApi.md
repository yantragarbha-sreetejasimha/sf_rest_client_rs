# \MeshApplicationsApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_application_create_or_update**](MeshApplicationsApi.md#mesh_application_create_or_update) | **Put** /Resources/Applications/{applicationResourceName} | Creates or updates a Application resource.
[**mesh_application_delete**](MeshApplicationsApi.md#mesh_application_delete) | **Delete** /Resources/Applications/{applicationResourceName} | Deletes the Application resource.
[**mesh_application_get**](MeshApplicationsApi.md#mesh_application_get) | **Get** /Resources/Applications/{applicationResourceName} | Gets the Application resource with the given name.
[**mesh_application_list**](MeshApplicationsApi.md#mesh_application_list) | **Get** /Resources/Applications | Lists all the application resources.


# **mesh_application_create_or_update**
> ::models::ApplicationResourceDescription mesh_application_create_or_update(api_version, application_resource_name, application_resource_description)
Creates or updates a Application resource.

Creates a Application resource with the specified name, description and properties. If Application resource with the same name exists, then it is updated with the specified description and properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 
  **application_resource_description** | [**ApplicationResourceDescription**](ApplicationResourceDescription.md)| Description for creating a Application resource. | 

### Return type

[**::models::ApplicationResourceDescription**](ApplicationResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_application_delete**
> mesh_application_delete(api_version, application_resource_name)
Deletes the Application resource.

Deletes the Application resource identified by the name.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_application_get**
> ::models::ApplicationResourceDescription mesh_application_get(api_version, application_resource_name)
Gets the Application resource with the given name.

Gets the information about the Application resource with the given name. The information include the description and other properties of the Application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 

### Return type

[**::models::ApplicationResourceDescription**](ApplicationResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_application_list**
> ::models::PagedApplicationResourceDescriptionList mesh_application_list(api_version)
Lists all the application resources.

Gets the information about all application resources in a given resource group. The information include the description and other properties of the Application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]

### Return type

[**::models::PagedApplicationResourceDescriptionList**](PagedApplicationResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

