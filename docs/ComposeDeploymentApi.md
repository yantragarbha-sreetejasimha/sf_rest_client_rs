# \ComposeDeploymentApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_compose_deployment**](ComposeDeploymentApi.md#create_compose_deployment) | **Put** /ComposeDeployments/$/Create | Creates a Service Fabric compose deployment.
[**get_compose_deployment_status**](ComposeDeploymentApi.md#get_compose_deployment_status) | **Get** /ComposeDeployments/{deploymentName} | Gets information about a Service Fabric compose deployment.
[**get_compose_deployment_status_list**](ComposeDeploymentApi.md#get_compose_deployment_status_list) | **Get** /ComposeDeployments | Gets the list of compose deployments created in the Service Fabric cluster.
[**get_compose_deployment_upgrade_progress**](ComposeDeploymentApi.md#get_compose_deployment_upgrade_progress) | **Get** /ComposeDeployments/{deploymentName}/$/GetUpgradeProgress | Gets details for the latest upgrade performed on this Service Fabric compose deployment.
[**remove_compose_deployment**](ComposeDeploymentApi.md#remove_compose_deployment) | **Post** /ComposeDeployments/{deploymentName}/$/Delete | Deletes an existing Service Fabric compose deployment from cluster.
[**start_compose_deployment_upgrade**](ComposeDeploymentApi.md#start_compose_deployment_upgrade) | **Post** /ComposeDeployments/{deploymentName}/$/Upgrade | Starts upgrading a compose deployment in the Service Fabric cluster.
[**start_rollback_compose_deployment_upgrade**](ComposeDeploymentApi.md#start_rollback_compose_deployment_upgrade) | **Post** /ComposeDeployments/{deploymentName}/$/RollbackUpgrade | Starts rolling back a compose deployment upgrade in the Service Fabric cluster.


# **create_compose_deployment**
> create_compose_deployment(api_version, create_compose_deployment_description, optional)
Creates a Service Fabric compose deployment.

Compose is a file format that describes multi-container applications. This API allows deploying container based applications defined in compose format in a Service Fabric cluster. Once the deployment is created, its status can be tracked via the `GetComposeDeploymentStatus` API.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
  **create_compose_deployment_description** | [**CreateComposeDeploymentDescription**](CreateComposeDeploymentDescription.md)| Describes the compose deployment that needs to be created. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **create_compose_deployment_description** | [**CreateComposeDeploymentDescription**](CreateComposeDeploymentDescription.md)| Describes the compose deployment that needs to be created. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_compose_deployment_status**
> ::models::ComposeDeploymentStatusInfo get_compose_deployment_status(api_version, deployment_name, optional)
Gets information about a Service Fabric compose deployment.

Returns the status of the compose deployment that was created or in the process of being created in the Service Fabric cluster and whose name matches the one specified as the parameter. The response includes the name, status, and other details about the deployment.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
  **deployment_name** | **String**| The identity of the deployment. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **deployment_name** | **String**| The identity of the deployment. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::ComposeDeploymentStatusInfo**](ComposeDeploymentStatusInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_compose_deployment_status_list**
> ::models::PagedComposeDeploymentStatusInfoList get_compose_deployment_status_list(api_version, optional)
Gets the list of compose deployments created in the Service Fabric cluster.

Gets the status about the compose deployments that were created or in the process of being created in the Service Fabric cluster. The response includes the name, status, and other details about the compose deployments. If the list of deployments do not fit in a page, one page of results is returned as well as a continuation token, which can be used to get the next page.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **continuation_token** | **String**| The continuation token parameter is used to obtain next set of results. A continuation token with a non-empty value is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token does not contain a value. The value of this parameter should not be URL encoded. | 
 **max_results** | **i64**| The maximum number of results to be returned as part of the paged queries. This parameter defines the upper bound on the number of results returned. The results returned can be less than the specified maximum results if they do not fit in the message as per the max message size restrictions defined in the configuration. If this parameter is zero or not specified, the paged query includes as many results as possible that fit in the return message. | [default to 0]
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::PagedComposeDeploymentStatusInfoList**](PagedComposeDeploymentStatusInfoList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_compose_deployment_upgrade_progress**
> ::models::ComposeDeploymentUpgradeProgressInfo get_compose_deployment_upgrade_progress(api_version, deployment_name, optional)
Gets details for the latest upgrade performed on this Service Fabric compose deployment.

Returns the information about the state of the compose deployment upgrade along with details to aid debugging application health issues.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
  **deployment_name** | **String**| The identity of the deployment. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **deployment_name** | **String**| The identity of the deployment. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::ComposeDeploymentUpgradeProgressInfo**](ComposeDeploymentUpgradeProgressInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_compose_deployment**
> remove_compose_deployment(api_version, deployment_name, optional)
Deletes an existing Service Fabric compose deployment from cluster.

Deletes an existing Service Fabric compose deployment.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
  **deployment_name** | **String**| The identity of the deployment. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **deployment_name** | **String**| The identity of the deployment. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **start_compose_deployment_upgrade**
> start_compose_deployment_upgrade(api_version, deployment_name, compose_deployment_upgrade_description, optional)
Starts upgrading a compose deployment in the Service Fabric cluster.

Validates the supplied upgrade parameters and starts upgrading the deployment if the parameters are valid.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
  **deployment_name** | **String**| The identity of the deployment. | 
  **compose_deployment_upgrade_description** | [**ComposeDeploymentUpgradeDescription**](ComposeDeploymentUpgradeDescription.md)| Parameters for upgrading compose deployment. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;\&quot;6.0-preview&#39;. | [default to 6.0-preview]
 **deployment_name** | **String**| The identity of the deployment. | 
 **compose_deployment_upgrade_description** | [**ComposeDeploymentUpgradeDescription**](ComposeDeploymentUpgradeDescription.md)| Parameters for upgrading compose deployment. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **start_rollback_compose_deployment_upgrade**
> start_rollback_compose_deployment_upgrade(api_version, deployment_name, optional)
Starts rolling back a compose deployment upgrade in the Service Fabric cluster.

Rollback a service fabric compose deployment upgrade.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **deployment_name** | **String**| The identity of the deployment. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
 **deployment_name** | **String**| The identity of the deployment. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

