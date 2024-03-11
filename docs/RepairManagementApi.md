# \RepairManagementApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_repair_task**](RepairManagementApi.md#cancel_repair_task) | **Post** /$/CancelRepairTask | Requests the cancellation of the given repair task.
[**create_repair_task**](RepairManagementApi.md#create_repair_task) | **Post** /$/CreateRepairTask | Creates a new repair task.
[**delete_repair_task**](RepairManagementApi.md#delete_repair_task) | **Post** /$/DeleteRepairTask | Deletes a completed repair task.
[**force_approve_repair_task**](RepairManagementApi.md#force_approve_repair_task) | **Post** /$/ForceApproveRepairTask | Forces the approval of the given repair task.
[**get_repair_task_list**](RepairManagementApi.md#get_repair_task_list) | **Get** /$/GetRepairTaskList | Gets a list of repair tasks matching the given filters.
[**update_repair_execution_state**](RepairManagementApi.md#update_repair_execution_state) | **Post** /$/UpdateRepairExecutionState | Updates the execution state of a repair task.
[**update_repair_task_health_policy**](RepairManagementApi.md#update_repair_task_health_policy) | **Post** /$/UpdateRepairTaskHealthPolicy | Updates the health policy of the given repair task.


# **cancel_repair_task**
> ::models::RepairTaskUpdateInfo cancel_repair_task(api_version, repair_task_cancel_description)
Requests the cancellation of the given repair task.

This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **repair_task_cancel_description** | [**RepairTaskCancelDescription**](RepairTaskCancelDescription.md)| Describes the repair task to be cancelled. | 

### Return type

[**::models::RepairTaskUpdateInfo**](RepairTaskUpdateInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_repair_task**
> ::models::RepairTaskUpdateInfo create_repair_task(api_version, repair_task)
Creates a new repair task.

For clusters that have the Repair Manager Service configured, this API provides a way to create repair tasks that run automatically or manually. For repair tasks that run automatically, an appropriate repair executor must be running for each repair action to run automatically. These are currently only available in specially-configured Azure Cloud Services.  To create a manual repair task, provide the set of impacted node names and the expected impact. When the state of the created repair task changes to approved, you can safely perform repair actions on those nodes.  This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **repair_task** | [**RepairTask**](RepairTask.md)| Describes the repair task to be created or updated. | 

### Return type

[**::models::RepairTaskUpdateInfo**](RepairTaskUpdateInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_repair_task**
> delete_repair_task(api_version, repair_task_delete_description)
Deletes a completed repair task.

This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **repair_task_delete_description** | [**RepairTaskDeleteDescription**](RepairTaskDeleteDescription.md)| Describes the repair task to be deleted. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **force_approve_repair_task**
> ::models::RepairTaskUpdateInfo force_approve_repair_task(api_version, repair_task_approve_description)
Forces the approval of the given repair task.

This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **repair_task_approve_description** | [**RepairTaskApproveDescription**](RepairTaskApproveDescription.md)| Describes the repair task to be approved. | 

### Return type

[**::models::RepairTaskUpdateInfo**](RepairTaskUpdateInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_repair_task_list**
> ::models::RepairTaskList get_repair_task_list(api_version, optional)
Gets a list of repair tasks matching the given filters.

This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
 **task_id_filter** | **String**| The repair task ID prefix to be matched. | 
 **state_filter** | **i32**| A bitwise-OR of the following values, specifying which task states should be included in the result list.  - 1 - Created - 2 - Claimed - 4 - Preparing - 8 - Approved - 16 - Executing - 32 - Restoring - 64 - Completed | 
 **executor_filter** | **String**| The name of the repair executor whose claimed tasks should be included in the list. | 

### Return type

[**::models::RepairTaskList**](RepairTaskList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_repair_execution_state**
> ::models::RepairTaskUpdateInfo update_repair_execution_state(api_version, repair_task)
Updates the execution state of a repair task.

This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **repair_task** | [**RepairTask**](RepairTask.md)| Describes the repair task to be created or updated. | 

### Return type

[**::models::RepairTaskUpdateInfo**](RepairTaskUpdateInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_repair_task_health_policy**
> ::models::RepairTaskUpdateInfo update_repair_task_health_policy(api_version, repair_task_update_health_policy_description)
Updates the health policy of the given repair task.

This API supports the Service Fabric platform; it is not meant to be used directly from your code.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.0&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accept any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0, but if the runtime is 6.1, in order to make it easier to write the clients, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.0]
  **repair_task_update_health_policy_description** | [**RepairTaskUpdateHealthPolicyDescription**](RepairTaskUpdateHealthPolicyDescription.md)| Describes the repair task healthy policy to be updated. | 

### Return type

[**::models::RepairTaskUpdateInfo**](RepairTaskUpdateInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

