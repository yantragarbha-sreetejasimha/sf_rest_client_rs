# GatewayProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | User readable description of the gateway. | [optional] [default to null]
**source_network** | [***::models::NetworkRef**](NetworkRef.md) | Network the gateway should listen on for requests. | [default to null]
**destination_network** | [***::models::NetworkRef**](NetworkRef.md) | Network that the Application is using. | [default to null]
**tcp** | [**Vec<::models::TcpConfig>**](TcpConfig.md) | Configuration for tcp connectivity for this gateway. | [optional] [default to null]
**http** | [**Vec<::models::HttpConfig>**](HttpConfig.md) | Configuration for http connectivity for this gateway. | [optional] [default to null]
**status** | [***::models::ResourceStatus**](ResourceStatus.md) | Status of the resource. | [optional] [default to null]
**status_details** | **String** | Gives additional information about the current status of the gateway. | [optional] [default to null]
**ip_address** | **String** | IP address of the gateway. This is populated in the response and is ignored for incoming requests. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


