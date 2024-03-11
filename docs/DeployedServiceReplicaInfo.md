# DeployedServiceReplicaInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | The full name of the service with &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**service_type_name** | [***::models::ServiceTypeName**](ServiceTypeName.md) | Name of the service type as specified in the service manifest. | [optional] [default to null]
**service_manifest_name** | [***::models::ServiceManifestName**](ServiceManifestName.md) | The name of the service manifest in which this service type is defined. | [optional] [default to null]
**code_package_name** | [***::models::CodePackageName**](CodePackageName.md) | The name of the code package that hosts this replica. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [optional] [default to null]
**replica_status** | [***::models::ReplicaStatus**](ReplicaStatus.md) | The status of a replica of a service. | [optional] [default to null]
**address** | **String** | The last address returned by the replica in Open or ChangeRole. | [optional] [default to null]
**service_package_activation_id** | [***::models::ServicePackageActivationId**](ServicePackageActivationId.md) | The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is &#39;SharedProcess&#39; (or if it is not specified, in which case it defaults to &#39;SharedProcess&#39;), then value of ServicePackageActivationId is always an empty string. | [optional] [default to null]
**host_process_id** | **String** | Host process ID of the process that is hosting the replica. This will be zero if the replica is down. In hyper-v containers this host process ID will be from different kernel. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


