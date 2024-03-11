# ChaosParameters

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_to_run_in_seconds** | **String** | Total time (in seconds) for which Chaos will run before automatically stopping. The maximum allowed value is 4,294,967,295 (System.UInt32.MaxValue). | [optional] [default to null]
**max_cluster_stabilization_timeout_in_seconds** | **i64** | The maximum amount of time to wait for all cluster entities to become stable and healthy. Chaos executes in iterations and at the start of each iteration it validates the health of cluster entities. During validation if a cluster entity is not stable and healthy within MaxClusterStabilizationTimeoutInSeconds, Chaos generates a validation failed event. | [optional] [default to 60]
**max_concurrent_faults** | **i64** | MaxConcurrentFaults is the maximum number of concurrent faults induced per iteration. Chaos executes in iterations and two consecutive iterations are separated by a validation phase. The higher the concurrency, the more aggressive the injection of faults, leading to inducing more complex series of states to uncover bugs. The recommendation is to start with a value of 2 or 3 and to exercise caution while moving up. | [optional] [default to 1]
**enable_move_replica_faults** | **bool** | Enables or disables the move primary and move secondary faults. | [optional] [default to null]
**wait_time_between_faults_in_seconds** | **i64** | Wait time (in seconds) between consecutive faults within a single iteration. The larger the value, the lower the overlapping between faults and the simpler the sequence of state transitions that the cluster goes through. The recommendation is to start with a value between 1 and 5 and exercise caution while moving up. | [optional] [default to 20]
**wait_time_between_iterations_in_seconds** | **i64** | Time-separation (in seconds) between two consecutive iterations of Chaos. The larger the value, the lower the fault injection rate. | [optional] [default to 30]
**cluster_health_policy** | [***::models::ClusterHealthPolicy**](ClusterHealthPolicy.md) | Passed-in cluster health policy is used to validate health of the cluster in between Chaos iterations. If the cluster health is in error or if an unexpected exception happens during fault execution--to provide the cluster with some time to recuperate--Chaos will wait for 30 minutes before the next health-check. | [optional] [default to null]
**context** | [***::models::ChaosContext**](ChaosContext.md) | Describes a map, which is a collection of (string, string) type key-value pairs. The map can be used to record information about the Chaos run. There cannot be more than 100 such pairs and each string (key or value) can be at most 4095 characters long. This map is set by the starter of the Chaos run to optionally store the context about the specific run. | [optional] [default to null]
**chaos_target_filter** | [***::models::ChaosTargetFilter**](ChaosTargetFilter.md) | List of cluster entities to target for Chaos faults. This filter can be used to target Chaos faults only to certain node types or only to certain application instances. If ChaosTargetFilter is not used, Chaos faults all cluster entities. If ChaosTargetFilter is used, Chaos faults only the entities that meet the ChaosTargetFilter specification. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


