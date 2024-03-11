# \ApplicationResourceApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application_resource**](ApplicationResourceApi.md#create_application_resource) | **Put** /Resources/Applications/{applicationResourceName} | Creates or updates an application resource.
[**delete_application_resource**](ApplicationResourceApi.md#delete_application_resource) | **Delete** /Resources/Applications/{applicationResourceName} | Deletes the specified application.
[**get_application_resource**](ApplicationResourceApi.md#get_application_resource) | **Get** /Resources/Applications/{applicationResourceName} | Gets the application with the given name.
[**get_replica**](ApplicationResourceApi.md#get_replica) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName}/Replicas/{replicaName} | Gets a specific replica of a given service in an application resource.
[**get_replicas**](ApplicationResourceApi.md#get_replicas) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName}/replicas | Gets replicas of a given service in an application resource.
[**get_service**](ApplicationResourceApi.md#get_service) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName} | Gets the description of the specified service in an application resource.
[**get_services**](ApplicationResourceApi.md#get_services) | **Get** /Resources/Applications/{applicationResourceName}/Services | Gets all the services in the application resource.


# **create_application_resource**
> create_application_resource(api_version, application_resource_name, application_resource_description)
Creates or updates an application resource.

Creates an application with the specified name and description. If an application with the same name already exists, then its description are updated to the one indicated in this request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 
  **application_resource_description** | [**ApplicationResourceDescription**](ApplicationResourceDescription.md)| Description for creating an application resource. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_application_resource**
> delete_application_resource(api_version, application_resource_name)
Deletes the specified application.

Deletes the application identified by the name.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_resource**
> ::models::ApplicationResourceDescription get_application_resource(api_version, application_resource_name)
Gets the application with the given name.

Gets the application with the given name. This includes the information about the application's services and other runtime information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 

### Return type

[**::models::ApplicationResourceDescription**](ApplicationResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_replica**
> ::models::ServiceResourceReplicaDescription get_replica(api_version, application_resource_name, service_resource_name, replica_name)
Gets a specific replica of a given service in an application resource.

Gets the information about the specified replica of a given service of an application. The information includes the runtime properties of the replica instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 
  **service_resource_name** | **String**| Service Fabric service resource name. | 
  **replica_name** | **String**| Service Fabric replica name. | 

### Return type

[**::models::ServiceResourceReplicaDescription**](ServiceResourceReplicaDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_replicas**
> ::models::PagedServiceResourceReplicaDescriptionList get_replicas(api_version, application_resource_name, service_resource_name)
Gets replicas of a given service in an application resource.

Gets the information about all replicas of a given service of an application. The information includes the runtime properties of the replica instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 
  **service_resource_name** | **String**| Service Fabric service resource name. | 

### Return type

[**::models::PagedServiceResourceReplicaDescriptionList**](PagedServiceResourceReplicaDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_service**
> ::models::ServiceResourceDescription get_service(api_version, application_resource_name, service_resource_name)
Gets the description of the specified service in an application resource.

Gets the description of the service resource.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 
  **service_resource_name** | **String**| Service Fabric service resource name. | 

### Return type

[**::models::ServiceResourceDescription**](ServiceResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_services**
> ::models::PagedServiceResourceDescriptionList get_services(api_version, application_resource_name)
Gets all the services in the application resource.

The operation returns the service descriptions of all the services in the application resource. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.3-preview&#39;. | [default to 6.3-preview]
  **application_resource_name** | **String**| Service Fabric application resource name. | 

### Return type

[**::models::PagedServiceResourceDescriptionList**](PagedServiceResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

