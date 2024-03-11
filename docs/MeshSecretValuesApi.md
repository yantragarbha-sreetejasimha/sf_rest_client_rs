# \MeshSecretValuesApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_secret_value_add_value**](MeshSecretValuesApi.md#mesh_secret_value_add_value) | **Put** /Resources/Secrets/{secretResourceName}/values/{secretValueResourceName} | Adds the specified value as a new version of the specified secret resource.
[**mesh_secret_value_delete**](MeshSecretValuesApi.md#mesh_secret_value_delete) | **Delete** /Resources/Secrets/{secretResourceName}/values/{secretValueResourceName} | Deletes the specified  value of the named secret resource.
[**mesh_secret_value_get**](MeshSecretValuesApi.md#mesh_secret_value_get) | **Get** /Resources/Secrets/{secretResourceName}/values/{secretValueResourceName} | Gets the specified secret value resource.
[**mesh_secret_value_list**](MeshSecretValuesApi.md#mesh_secret_value_list) | **Get** /Resources/Secrets/{secretResourceName}/values | List names of all values of the specified secret resource.
[**mesh_secret_value_show**](MeshSecretValuesApi.md#mesh_secret_value_show) | **Post** /Resources/Secrets/{secretResourceName}/values/{secretValueResourceName}/list_value | Lists the specified value of the secret resource.


# **mesh_secret_value_add_value**
> ::models::SecretValueResourceDescription mesh_secret_value_add_value(api_version, secret_resource_name, secret_value_resource_name, secret_value_resource_description)
Adds the specified value as a new version of the specified secret resource.

Creates a new value of the specified secret resource. The name of the value is typically the version identifier. Once created the value cannot be changed.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 
  **secret_value_resource_name** | **String**| The name of the secret resource value which is typically the version identifier for the value. | 
  **secret_value_resource_description** | [**SecretValueResourceDescription**](SecretValueResourceDescription.md)| Description for creating a value of a secret resource. | 

### Return type

[**::models::SecretValueResourceDescription**](SecretValueResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_value_delete**
> mesh_secret_value_delete(api_version, secret_resource_name, secret_value_resource_name)
Deletes the specified  value of the named secret resource.

Deletes the secret value resource identified by the name. The name of the resource is typically the version associated with that value. Deletion will fail if the specified value is in use.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 
  **secret_value_resource_name** | **String**| The name of the secret resource value which is typically the version identifier for the value. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_value_get**
> ::models::SecretValueResourceDescription mesh_secret_value_get(api_version, secret_resource_name, secret_value_resource_name)
Gets the specified secret value resource.

Get the information about the specified named secret value resources. The information does not include the actual value of the secret.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 
  **secret_value_resource_name** | **String**| The name of the secret resource value which is typically the version identifier for the value. | 

### Return type

[**::models::SecretValueResourceDescription**](SecretValueResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_value_list**
> ::models::PagedSecretValueResourceDescriptionList mesh_secret_value_list(api_version, secret_resource_name)
List names of all values of the specified secret resource.

Gets information about all secret value resources of the specified secret resource. The information includes the names of the secret value resources, but not the actual values.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 

### Return type

[**::models::PagedSecretValueResourceDescriptionList**](PagedSecretValueResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_value_show**
> ::models::SecretValue mesh_secret_value_show(api_version, secret_resource_name, secret_value_resource_name)
Lists the specified value of the secret resource.

Lists the decrypted value of the specified named value of the secret resource. This is a privileged operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 
  **secret_value_resource_name** | **String**| The name of the secret resource value which is typically the version identifier for the value. | 

### Return type

[**::models::SecretValue**](SecretValue.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

