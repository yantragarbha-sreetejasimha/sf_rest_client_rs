# \MeshCodePackagesApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_code_package_get_container_logs**](MeshCodePackagesApi.md#mesh_code_package_get_container_logs) | **Get** /Resources/Applications/{applicationResourceName}/Services/{serviceResourceName}/Replicas/{replicaName}/CodePackages/{codePackageName}/Logs | Gets the logs from the container.


# **mesh_code_package_get_container_logs**
> ::models::ContainerLogs mesh_code_package_get_container_logs(api_version, application_resource_name, service_resource_name, replica_name, code_package_name, optional)
Gets the logs from the container.

Gets the logs for the container of the specified code package of the service replica.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **application_resource_name** | **String**| The identity of the application. | 
  **service_resource_name** | **String**| The identity of the service. | 
  **replica_name** | **String**| Service Fabric replica name. | 
  **code_package_name** | **String**| The name of code package of the service. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
 **application_resource_name** | **String**| The identity of the application. | 
 **service_resource_name** | **String**| The identity of the service. | 
 **replica_name** | **String**| Service Fabric replica name. | 
 **code_package_name** | **String**| The name of code package of the service. | 
 **tail** | **String**| Number of lines to show from the end of the logs. Default is 100. &#39;all&#39; to show the complete logs. | 

### Return type

[**::models::ContainerLogs**](ContainerLogs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

