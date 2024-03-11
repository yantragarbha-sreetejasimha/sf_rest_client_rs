# ChaosStartedEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**category** | **String** | The category of event. | [optional] [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**max_concurrent_faults** | **i64** | Maximum number of concurrent faults. | [default to null]
**time_to_run_in_seconds** | **f64** | Time to run in seconds. | [default to null]
**max_cluster_stabilization_timeout_in_seconds** | **f64** | Maximum timeout for cluster stabilization in seconds. | [default to null]
**wait_time_between_iterations_in_seconds** | **f64** | Wait time between iterations in seconds. | [default to null]
**wait_time_between_faults_in_seconds** | **f64** | Wait time between faults in seconds. | [default to null]
**move_replica_fault_enabled** | **bool** | Indicates MoveReplica fault is enabled. | [default to null]
**included_node_type_list** | **String** | List of included Node types. | [default to null]
**included_application_list** | **String** | List of included Applications. | [default to null]
**cluster_health_policy** | **String** | Health policy. | [default to null]
**chaos_context** | **String** | Chaos Context. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


