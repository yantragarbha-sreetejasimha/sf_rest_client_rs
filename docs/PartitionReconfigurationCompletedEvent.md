# PartitionReconfigurationCompletedEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [default to null]
**node_name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [default to null]
**node_instance_id** | **String** | Id of Node instance. | [default to null]
**service_type** | **String** | Type of Service. | [default to null]
**cc_epoch_data_loss_version** | **i64** | CcEpochDataLoss version. | [default to null]
**cc_epoch_config_version** | **i64** | CcEpochConfig version. | [default to null]
**reconfig_type** | **String** | Type of reconfiguration. | [default to null]
**result** | **String** | Describes reconfiguration result. | [default to null]
**phase0_duration_ms** | **f64** | Duration of Phase0 in milli-seconds. | [default to null]
**phase1_duration_ms** | **f64** | Duration of Phase1 in milli-seconds. | [default to null]
**phase2_duration_ms** | **f64** | Duration of Phase2 in milli-seconds. | [default to null]
**phase3_duration_ms** | **f64** | Duration of Phase3 in milli-seconds. | [default to null]
**phase4_duration_ms** | **f64** | Duration of Phase4 in milli-seconds. | [default to null]
**total_duration_ms** | **f64** | Total duration in milli-seconds. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


