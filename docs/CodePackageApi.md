# \CodePackageApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_container_logs_deployed_on_node**](CodePackageApi.md#get_container_logs_deployed_on_node) | **Get** /Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetCodePackages/$/ContainerLogs | Gets the container logs for container deployed on a Service Fabric node.
[**get_deployed_code_package_info_list**](CodePackageApi.md#get_deployed_code_package_info_list) | **Get** /Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetCodePackages | Gets the list of code packages deployed on a Service Fabric node.
[**invoke_container_api**](CodePackageApi.md#invoke_container_api) | **Post** /Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetCodePackages/$/ContainerApi | Invoke container API on a container deployed on a Service Fabric node.
[**restart_deployed_code_package**](CodePackageApi.md#restart_deployed_code_package) | **Post** /Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetCodePackages/$/Restart | Restarts a code package deployed on a Service Fabric node in a cluster.


# **get_container_logs_deployed_on_node**
> ::models::ContainerLogs get_container_logs_deployed_on_node(api_version, node_name, application_id, service_manifest_name, code_package_name, optional)
Gets the container logs for container deployed on a Service Fabric node.

Gets the container logs for container deployed on a Service Fabric node for the given code package.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.2&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.2]
  **node_name** | **String**| The name of the node. | 
  **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
  **service_manifest_name** | **String**| The name of a service manifest registered as part of an application type in a Service Fabric cluster. | 
  **code_package_name** | **String**| The name of code package specified in service manifest registered as part of an application type in a Service Fabric cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.2&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.2]
 **node_name** | **String**| The name of the node. | 
 **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
 **service_manifest_name** | **String**| The name of a service manifest registered as part of an application type in a Service Fabric cluster. | 
 **code_package_name** | **String**| The name of code package specified in service manifest registered as part of an application type in a Service Fabric cluster. | 
 **tail** | **String**| Number of lines to show from the end of the logs. Default is 100. &#39;all&#39; to show the complete logs. | 
 **previous** | **bool**| Specifies whether to get container logs from exited/dead containers of the code package instance. | [default to false]
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::ContainerLogs**](ContainerLogs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_deployed_code_package_info_list**
> ::models::DeployedCodePackageInfoList get_deployed_code_package_info_list(api_version, node_name, application_id, optional)
Gets the list of code packages deployed on a Service Fabric node.

Gets the list of code packages deployed on a Service Fabric node for the given application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **node_name** | **String**| The name of the node. | 
  **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
 **node_name** | **String**| The name of the node. | 
 **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
 **service_manifest_name** | **String**| The name of a service manifest registered as part of an application type in a Service Fabric cluster. | 
 **code_package_name** | **String**| The name of code package specified in service manifest registered as part of an application type in a Service Fabric cluster. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::DeployedCodePackageInfoList**](DeployedCodePackageInfoList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **invoke_container_api**
> ::models::ContainerApiResponse invoke_container_api(api_version, node_name, application_id, service_manifest_name, code_package_name, code_package_instance_id, container_api_request_body, optional)
Invoke container API on a container deployed on a Service Fabric node.

Invoke container API on a container deployed on a Service Fabric node for the given code package.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.2&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.2]
  **node_name** | **String**| The name of the node. | 
  **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
  **service_manifest_name** | **String**| The name of a service manifest registered as part of an application type in a Service Fabric cluster. | 
  **code_package_name** | **String**| The name of code package specified in service manifest registered as part of an application type in a Service Fabric cluster. | 
  **code_package_instance_id** | **String**| ID that uniquely identifies a code package instance deployed on a service fabric node. | 
  **container_api_request_body** | [**ContainerApiRequestBody**](ContainerApiRequestBody.md)| Parameters for making container API call | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.2&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.2]
 **node_name** | **String**| The name of the node. | 
 **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
 **service_manifest_name** | **String**| The name of a service manifest registered as part of an application type in a Service Fabric cluster. | 
 **code_package_name** | **String**| The name of code package specified in service manifest registered as part of an application type in a Service Fabric cluster. | 
 **code_package_instance_id** | **String**| ID that uniquely identifies a code package instance deployed on a service fabric node. | 
 **container_api_request_body** | [**ContainerApiRequestBody**](ContainerApiRequestBody.md)| Parameters for making container API call | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::ContainerApiResponse**](ContainerApiResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **restart_deployed_code_package**
> restart_deployed_code_package(api_version, node_name, application_id, restart_deployed_code_package_description, optional)
Restarts a code package deployed on a Service Fabric node in a cluster.

Restarts a code package deployed on a Service Fabric node in a cluster. This aborts the code package process, which will restart all the user service replicas hosted in that process.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **node_name** | **String**| The name of the node. | 
  **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
  **restart_deployed_code_package_description** | [**RestartDeployedCodePackageDescription**](RestartDeployedCodePackageDescription.md)| Describes the deployed code package on Service Fabric node to restart. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
 **node_name** | **String**| The name of the node. | 
 **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
 **restart_deployed_code_package_description** | [**RestartDeployedCodePackageDescription**](RestartDeployedCodePackageDescription.md)| Describes the deployed code package on Service Fabric node to restart. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

