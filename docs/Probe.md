# Probe

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**initial_delay_seconds** | **i32** | The initial delay in seconds to start executing probe once code package has started. | [optional] [default to null]
**period_seconds** | **i32** | Periodic seconds to execute probe. | [optional] [default to null]
**timeout_seconds** | **i32** | Period after which probe is considered as failed if it hasn&#39;t completed successfully. | [optional] [default to null]
**success_threshold** | **i32** | The count of successful probe executions after which probe is considered success. | [optional] [default to null]
**failure_threshold** | **i32** | The count of failures after which probe is considered failed. | [optional] [default to null]
**exec** | [***::models::ProbeExec**](ProbeExec.md) | Exec command to run inside the container. | [optional] [default to null]
**http_get** | [***::models::ProbeHttpGet**](ProbeHttpGet.md) | Http probe for the container. | [optional] [default to null]
**tcp_socket** | [***::models::ProbeTcpSocket**](ProbeTcpSocket.md) | Tcp port to probe inside the container. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


