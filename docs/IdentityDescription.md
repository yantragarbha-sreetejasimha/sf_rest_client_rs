# IdentityDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token_service_endpoint** | **String** | the endpoint for the token service managing this identity | [optional] [default to null]
**_type** | **String** | the types of identities associated with this resource; currently restricted to &#39;SystemAssigned and UserAssigned&#39; | [default to null]
**tenant_id** | **String** | the identifier of the tenant containing the application&#39;s identity. | [optional] [default to null]
**principal_id** | **String** | the object identifier of the Service Principal of the identity associated with this resource. | [optional] [default to null]
**user_assigned_identities** | [***::models::UserAssignedIdentityMap**](UserAssignedIdentityMap.md) | represents user assigned identities map. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


