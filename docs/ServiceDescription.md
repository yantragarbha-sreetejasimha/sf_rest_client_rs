# ServiceDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The service kind. | [default to null]
**application_name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | The full name of the service with &#39;fabric:&#39; URI scheme. | [default to null]
**service_type_name** | [***::models::ServiceTypeName**](ServiceTypeName.md) | Name of the service type as specified in the service manifest. | [default to null]
**initialization_data** | **String** | The initialization data as an array of bytes. Initialization data is passed to service instances or replicas when they are created. | [optional] [default to null]
**partition_description** | [***::models::PartitionSchemeDescription**](PartitionSchemeDescription.md) | The partition description as an object. | [default to null]
**placement_constraints** | **String** | The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \&quot;NodeColor &#x3D;&#x3D; blue)\&quot;. | [optional] [default to null]
**correlation_scheme** | [***::models::CorrelationSchemeList**](CorrelationSchemeList.md) | The correlation scheme. | [optional] [default to null]
**service_load_metrics** | [***::models::ServiceLoadMetricsList**](ServiceLoadMetricsList.md) | The service load metrics. | [optional] [default to null]
**service_placement_policies** | [***::models::ServicePlacementPoliciesList**](ServicePlacementPoliciesList.md) | The service placement policies. | [optional] [default to null]
**default_move_cost** | [***::models::MoveCost**](MoveCost.md) | The move cost for the service. | [optional] [default to null]
**is_default_move_cost_specified** | **bool** | Indicates if the DefaultMoveCost property is specified. | [optional] [default to null]
**service_package_activation_mode** | [***::models::ServicePackageActivationMode**](ServicePackageActivationMode.md) | The activation mode of service package to be used for a service. | [optional] [default to null]
**service_dns_name** | **String** | The DNS name of the service. It requires the DNS system service to be enabled in Service Fabric cluster. | [optional] [default to null]
**scaling_policies** | [***::models::ScalingPolicyDescriptionList**](ScalingPolicyDescriptionList.md) | Scaling policies for this service. | [optional] [default to null]
**tags_required_to_place** | [***::models::NodeTagsDescription**](NodeTagsDescription.md) | Tags for placement of this service. | [optional] [default to null]
**tags_required_to_run** | [***::models::NodeTagsDescription**](NodeTagsDescription.md) | Tags for running of this service. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


