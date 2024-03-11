# \EventsStoreApi

All URIs are relative to *http://localhost:19080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_application_event_list**](EventsStoreApi.md#get_application_event_list) | **Get** /EventsStore/Applications/{applicationId}/$/Events | Gets an Application-related events.
[**get_applications_event_list**](EventsStoreApi.md#get_applications_event_list) | **Get** /EventsStore/Applications/Events | Gets all Applications-related events.
[**get_cluster_event_list**](EventsStoreApi.md#get_cluster_event_list) | **Get** /EventsStore/Cluster/Events | Gets all Cluster-related events.
[**get_containers_event_list**](EventsStoreApi.md#get_containers_event_list) | **Get** /EventsStore/Containers/Events | Gets all Containers-related events.
[**get_correlated_event_list**](EventsStoreApi.md#get_correlated_event_list) | **Get** /EventsStore/CorrelatedEvents/{eventInstanceId}/$/Events | Gets all correlated events for a given event.
[**get_node_event_list**](EventsStoreApi.md#get_node_event_list) | **Get** /EventsStore/Nodes/{nodeName}/$/Events | Gets a Node-related events.
[**get_nodes_event_list**](EventsStoreApi.md#get_nodes_event_list) | **Get** /EventsStore/Nodes/Events | Gets all Nodes-related Events.
[**get_partition_event_list**](EventsStoreApi.md#get_partition_event_list) | **Get** /EventsStore/Partitions/{partitionId}/$/Events | Gets a Partition-related events.
[**get_partition_replica_event_list**](EventsStoreApi.md#get_partition_replica_event_list) | **Get** /EventsStore/Partitions/{partitionId}/$/Replicas/{replicaId}/$/Events | Gets a Partition Replica-related events.
[**get_partition_replicas_event_list**](EventsStoreApi.md#get_partition_replicas_event_list) | **Get** /EventsStore/Partitions/{partitionId}/$/Replicas/Events | Gets all Replicas-related events for a Partition.
[**get_partitions_event_list**](EventsStoreApi.md#get_partitions_event_list) | **Get** /EventsStore/Partitions/Events | Gets all Partitions-related events.
[**get_service_event_list**](EventsStoreApi.md#get_service_event_list) | **Get** /EventsStore/Services/{serviceId}/$/Events | Gets a Service-related events.
[**get_services_event_list**](EventsStoreApi.md#get_services_event_list) | **Get** /EventsStore/Services/Events | Gets all Services-related events.


# **get_application_event_list**
> ::models::ApplicationEventList get_application_event_list(api_version, application_id, start_time_utc, end_time_utc, optional)
Gets an Application-related events.

The response is list of ApplicationEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **application_id** | **String**| The identity of the application. This is typically the full name of the application without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | 
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ApplicationEventList**](ApplicationEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_applications_event_list**
> ::models::ApplicationEventList get_applications_event_list(api_version, start_time_utc, end_time_utc, optional)
Gets all Applications-related events.

The response is list of ApplicationEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ApplicationEventList**](ApplicationEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_event_list**
> ::models::ClusterEventList get_cluster_event_list(api_version, start_time_utc, end_time_utc, optional)
Gets all Cluster-related events.

The response is list of ClusterEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ClusterEventList**](ClusterEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_containers_event_list**
> ::models::ContainerInstanceEventList get_containers_event_list(api_version, start_time_utc, end_time_utc, optional)
Gets all Containers-related events.

The response is list of ContainerInstanceEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.2-preview&#39;. | [default to 6.2-preview]
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.2-preview&#39;. | [default to 6.2-preview]
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ContainerInstanceEventList**](ContainerInstanceEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_correlated_event_list**
> ::models::EventList get_correlated_event_list(api_version, event_instance_id, optional)
Gets all correlated events for a given event.

The response is list of FabricEvents.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **event_instance_id** | **String**| The EventInstanceId. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **event_instance_id** | **String**| The EventInstanceId. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]

### Return type

[**::models::EventList**](EventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_event_list**
> ::models::NodeEventList get_node_event_list(api_version, node_name, start_time_utc, end_time_utc, optional)
Gets a Node-related events.

The response is list of NodeEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **node_name** | **String**| The name of the node. | 
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **node_name** | **String**| The name of the node. | 
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::NodeEventList**](NodeEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nodes_event_list**
> ::models::NodeEventList get_nodes_event_list(api_version, start_time_utc, end_time_utc, optional)
Gets all Nodes-related Events.

The response is list of NodeEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::NodeEventList**](NodeEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_partition_event_list**
> ::models::PartitionEventList get_partition_event_list(api_version, partition_id, start_time_utc, end_time_utc, optional)
Gets a Partition-related events.

The response is list of PartitionEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **partition_id** | [**String**](.md)| The identity of the partition. | 
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **partition_id** | [**String**](.md)| The identity of the partition. | 
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::PartitionEventList**](PartitionEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_partition_replica_event_list**
> ::models::ReplicaEventList get_partition_replica_event_list(api_version, partition_id, replica_id, start_time_utc, end_time_utc, optional)
Gets a Partition Replica-related events.

The response is list of ReplicaEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **partition_id** | [**String**](.md)| The identity of the partition. | 
  **replica_id** | **String**| The identifier of the replica. | 
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **partition_id** | [**String**](.md)| The identity of the partition. | 
 **replica_id** | **String**| The identifier of the replica. | 
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ReplicaEventList**](ReplicaEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_partition_replicas_event_list**
> ::models::ReplicaEventList get_partition_replicas_event_list(api_version, partition_id, start_time_utc, end_time_utc, optional)
Gets all Replicas-related events for a Partition.

The response is list of ReplicaEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **partition_id** | [**String**](.md)| The identity of the partition. | 
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **partition_id** | [**String**](.md)| The identity of the partition. | 
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ReplicaEventList**](ReplicaEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_partitions_event_list**
> ::models::PartitionEventList get_partitions_event_list(api_version, start_time_utc, end_time_utc, optional)
Gets all Partitions-related events.

The response is list of PartitionEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::PartitionEventList**](PartitionEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_service_event_list**
> ::models::ServiceEventList get_service_event_list(api_version, service_id, start_time_utc, end_time_utc, optional)
Gets a Service-related events.

The response is list of ServiceEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **service_id** | **String**| The identity of the service. This ID is typically the full name of the service without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the service name is \&quot;fabric:/myapp/app1/svc1\&quot;, the service identity would be \&quot;myapp~app1~svc1\&quot; in 6.0+ and \&quot;myapp/app1/svc1\&quot; in previous versions. | 
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **service_id** | **String**| The identity of the service. This ID is typically the full name of the service without the &#39;fabric:&#39; URI scheme. Starting from version 6.0, hierarchical names are delimited with the \&quot;~\&quot; character. For example, if the service name is \&quot;fabric:/myapp/app1/svc1\&quot;, the service identity would be \&quot;myapp~app1~svc1\&quot; in 6.0+ and \&quot;myapp/app1/svc1\&quot; in previous versions. | 
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ServiceEventList**](ServiceEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_services_event_list**
> ::models::ServiceEventList get_services_event_list(api_version, start_time_utc, end_time_utc, optional)
Gets all Services-related events.

The response is list of ServiceEvent objects.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
  **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
  **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_version** | **String**| The version of the API. This parameter is required and its value must be &#39;6.4&#39;.  Service Fabric REST API version is based on the runtime version in which the API was introduced or was changed. Service Fabric runtime supports more than one version of the API. This version is the latest supported version of the API. If a lower API version is passed, the returned response may be different from the one documented in this specification.  Additionally the runtime accepts any version that is higher than the latest supported version up to the current version of the runtime. So if the latest API version is 6.0 and the runtime is 6.1, the runtime will accept version 6.1 for that API. However the behavior of the API will be as per the documented 6.0 version. | [default to 6.4]
 **start_time_utc** | **String**| The start time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **end_time_utc** | **String**| The end time of a lookup query in ISO UTC yyyy-MM-ddTHH:mm:ssZ. | 
 **timeout** | **i64**| The server timeout for performing the operation in seconds. This timeout specifies the time duration that the client is willing to wait for the requested operation to complete. The default value for this parameter is 60 seconds. | [default to 60]
 **events_types_filter** | **String**| This is a comma separated string specifying the types of FabricEvents that should only be included in the response. | 
 **exclude_analysis_events** | **bool**| This param disables the retrieval of AnalysisEvents if true is passed. | 
 **skip_correlation_lookup** | **bool**| This param disables the search of CorrelatedEvents information if true is passed. otherwise the CorrelationEvents get processed and HasCorrelatedEvents field in every FabricEvent gets populated. | 

### Return type

[**::models::ServiceEventList**](ServiceEventList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

