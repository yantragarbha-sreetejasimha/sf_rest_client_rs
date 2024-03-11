# \MeshSecretsApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mesh_secret_create_or_update**](MeshSecretsApi.md#mesh_secret_create_or_update) | **Put** /Resources/Secrets/{secretResourceName} | Creates or updates a Secret resource.
[**mesh_secret_delete**](MeshSecretsApi.md#mesh_secret_delete) | **Delete** /Resources/Secrets/{secretResourceName} | Deletes the Secret resource.
[**mesh_secret_get**](MeshSecretsApi.md#mesh_secret_get) | **Get** /Resources/Secrets/{secretResourceName} | Gets the Secret resource with the given name.
[**mesh_secret_list**](MeshSecretsApi.md#mesh_secret_list) | **Get** /Resources/Secrets | Lists all the secret resources.


# **mesh_secret_create_or_update**
> ::models::SecretResourceDescription mesh_secret_create_or_update(api_version, secret_resource_name, secret_resource_description)
Creates or updates a Secret resource.

Creates a Secret resource with the specified name, description and properties. If Secret resource with the same name exists, then it is updated with the specified description and properties. Once created, the kind and contentType of a secret resource cannot be updated.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 
  **secret_resource_description** | [**SecretResourceDescription**](SecretResourceDescription.md)| Description for creating a secret resource. | 

### Return type

[**::models::SecretResourceDescription**](SecretResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_delete**
> mesh_secret_delete(api_version, secret_resource_name)
Deletes the Secret resource.

Deletes the specified Secret resource and all of its named values.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_get**
> ::models::SecretResourceDescription mesh_secret_get(api_version, secret_resource_name)
Gets the Secret resource with the given name.

Gets the information about the Secret resource with the given name. The information include the description and other properties of the Secret.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]
  **secret_resource_name** | **String**| The name of the secret resource. | 

### Return type

[**::models::SecretResourceDescription**](SecretResourceDescription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mesh_secret_list**
> ::models::PagedSecretResourceDescriptionList mesh_secret_list(api_version)
Lists all the secret resources.

Gets the information about all secret resources in a given resource group. The information include the description and other properties of the Secret.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4-preview&#39;. | [default to 6.4-preview]

### Return type

[**::models::PagedSecretResourceDescriptionList**](PagedSecretResourceDescriptionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

