# NodeTypeNodesHealthEvaluation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::HealthEvaluationKind**](HealthEvaluationKind.md) | The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values. | [default to null]
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**description** | **String** | Description of the health evaluation, which represents a summary of the evaluation process. | [optional] [default to null]
**node_type_name** | [***::models::NodeTypeName**](NodeTypeName.md) | The node type name as defined in the cluster manifest. | [optional] [default to null]
**max_percent_unhealthy_nodes** | **i32** | Maximum allowed percentage of unhealthy nodes for the node type, specified as an entry in NodeTypeHealthPolicyMap. | [optional] [default to null]
**total_count** | **i64** | Total number of nodes of the node type found in the health store. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | List of unhealthy evaluations that led to the aggregated health state. Includes all the unhealthy NodeHealthEvaluation of this node type that impacted the aggregated health. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

