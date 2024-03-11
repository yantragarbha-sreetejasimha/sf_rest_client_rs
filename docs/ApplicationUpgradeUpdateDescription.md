# ApplicationUpgradeUpdateDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [default to null]
**upgrade_kind** | [***::models::UpgradeKind**](UpgradeKind.md) | The kind of upgrade out of the following possible values. | [default to null]
**application_health_policy** | [***::models::ApplicationHealthPolicy**](ApplicationHealthPolicy.md) | Defines a health policy used to evaluate the health of an application or one of its children entities. | [optional] [default to null]
**update_description** | [***::models::RollingUpgradeUpdateDescription**](RollingUpgradeUpdateDescription.md) | Describes the parameters for updating a rolling upgrade of application or cluster. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


