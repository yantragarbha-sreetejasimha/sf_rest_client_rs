# UpgradeDomainDeltaNodesCheckHealthEvaluation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::HealthEvaluationKind**](HealthEvaluationKind.md) | The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values. | [default to null]
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**description** | **String** | Description of the health evaluation, which represents a summary of the evaluation process. | [optional] [default to null]
**upgrade_domain_name** | **String** | Name of the upgrade domain where nodes health is currently evaluated. | [optional] [default to null]
**baseline_error_count** | **i64** | Number of upgrade domain nodes with aggregated heath state Error in the health store at the beginning of the cluster upgrade. | [optional] [default to null]
**baseline_total_count** | **i64** | Total number of upgrade domain nodes in the health store at the beginning of the cluster upgrade. | [optional] [default to null]
**max_percent_delta_unhealthy_nodes** | **i32** | Maximum allowed percentage of upgrade domain delta unhealthy nodes from the ClusterUpgradeHealthPolicy. | [optional] [default to null]
**total_count** | **i64** | Total number of upgrade domain nodes in the health store. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | List of unhealthy evaluations that led to the aggregated health state. Includes all the unhealthy NodeHealthEvaluation that impacted the aggregated health. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


