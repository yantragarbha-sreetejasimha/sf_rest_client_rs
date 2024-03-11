# \MeshServiceReplicasApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_service_replica_get**](MeshServiceReplicasApi.md#mesh_service_replica_get) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName}/Replicas/{replicaName} | Gets the given replica of the service of an application.
[**mesh_service_replica_list**](MeshServiceReplicasApi.md#mesh_service_replica_list) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName}/Replicas | Lists all the replicas of a service.


# **mesh_service_replica_get**
> ::models::ServiceReplicaDescription mesh_service_replica_get(api_version, application_resource_name, service_resource_name, replica_name)
Gets the given replica of the service of an application.

Gets the information about the service replica with the given name. The information include the description and other properties of the service replica.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 
  **service_resource_name** | **String**| The identity of the service. | 
  **replica_name** | **String**| Service Fabric replica name. | 

### Return type

[**::models::ServiceReplicaDescription**](ServiceReplicaDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_service_replica_list**
> ::models::PagedServiceReplicaDescriptionList mesh_service_replica_list(api_version, application_resource_name, service_resource_name)
Lists all the replicas of a service.

Gets the information about all replicas of a service. The information include the description and other properties of the service replica.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 
  **service_resource_name** | **String**| The identity of the service. | 

### Return type

[**::models::PagedServiceReplicaDescriptionList**](PagedServiceReplicaDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

