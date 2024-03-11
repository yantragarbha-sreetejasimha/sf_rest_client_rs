# PartitionLoadInformation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**partition_id** | [***::models::PartitionId**](PartitionId.md) | Id of the partition. | [optional] [default to null]
**primary_load_metric_reports** | [**Vec<::models::LoadMetricReport>**](LoadMetricReport.md) | Array of load reports from the primary replica for this partition. | [optional] [default to null]
**secondary_load_metric_reports** | [**Vec<::models::LoadMetricReport>**](LoadMetricReport.md) | Array of aggregated load reports from all secondary replicas for this partition. Array only contains the latest reported load for each metric. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


