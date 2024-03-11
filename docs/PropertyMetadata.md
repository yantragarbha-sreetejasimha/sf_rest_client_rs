# PropertyMetadata

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_id** | [***::models::PropertyValueKind**](PropertyValueKind.md) | The kind of property, determined by the type of data. Following are the possible values. | [optional] [default to null]
**custom_type_id** | [***::models::PropertyCustomTypeId**](PropertyCustomTypeId.md) | The property&#39;s custom type ID. | [optional] [default to null]
**parent** | [***::models::FabricName**](FabricName.md) | The name of the parent Service Fabric Name for the property. It could be thought of as the name-space/table under which the property exists. | [optional] [default to null]
**size_in_bytes** | **i32** | The length of the serialized property value. | [optional] [default to null]
**last_modified_utc_timestamp** | **String** | Represents when the Property was last modified. Only write operations will cause this field to be updated. | [optional] [default to null]
**sequence_number** | **String** | The version of the property. Every time a property is modified, its sequence number is increased. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


