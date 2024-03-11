# PartitionMetricLoadDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**partition_id** | [***::models::PartitionId**](PartitionId.md) | Id of the partition. | [optional] [default to null]
**primary_replica_load_entries** | [**Vec<::models::MetricLoadDescription>**](MetricLoadDescription.md) | Partition&#39;s load information for primary replica, in case partition is from a stateful service. | [optional] [default to null]
**secondary_replicas_or_instances_load_entries** | [**Vec<::models::MetricLoadDescription>**](MetricLoadDescription.md) | Partition&#39;s load information for all secondary replicas or instances. | [optional] [default to null]
**secondary_replica_or_instance_load_entries_per_node** | [**Vec<::models::ReplicaMetricLoadDescription>**](ReplicaMetricLoadDescription.md) | Partition&#39;s load information for a specific secondary replica or instance located on a specific node. | [optional] [default to null]
**auxiliary_replicas_load_entries** | [**Vec<::models::MetricLoadDescription>**](MetricLoadDescription.md) | Partition&#39;s load information for all auxiliary replicas. | [optional] [default to null]
**auxiliary_replica_load_entries_per_node** | [**Vec<::models::ReplicaMetricLoadDescription>**](ReplicaMetricLoadDescription.md) | Partition&#39;s load information for a specific auxiliary replica located on a specific node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


