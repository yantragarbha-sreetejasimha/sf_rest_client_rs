/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceLoadMetricDescription : Specifies a metric to load balance a service during runtime.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceLoadMetricDescription {
    /// The name of the metric. If the service chooses to report load during runtime, the load metric name should match the name that is specified in Name exactly. Note that metric names are case-sensitive.
    #[serde(rename = "Name")]
    name: String,
    /// The service load metric relative weight, compared to other metrics configured for this service, as a number.
    #[serde(rename = "Weight")]
    weight: Option<::models::ServiceLoadMetricWeight>,
    /// Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Primary replica.
    #[serde(rename = "PrimaryDefaultLoad")]
    primary_default_load: Option<i32>,
    /// Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Secondary replica.
    #[serde(rename = "SecondaryDefaultLoad")]
    secondary_default_load: Option<i32>,
    /// Used only for Stateless services. The default amount of load, as a number, that this service creates for this metric.
    #[serde(rename = "DefaultLoad")]
    default_load: Option<i32>,
}

impl ServiceLoadMetricDescription {
    /// Specifies a metric to load balance a service during runtime.
    pub fn new(name: String) -> ServiceLoadMetricDescription {
        ServiceLoadMetricDescription {
            name,
            weight: None,
            primary_default_load: None,
            secondary_default_load: None,
            default_load: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> ServiceLoadMetricDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_weight(&mut self, weight: ::models::ServiceLoadMetricWeight) {
        self.weight = Some(weight);
    }

    pub fn with_weight(
        mut self,
        weight: ::models::ServiceLoadMetricWeight,
    ) -> ServiceLoadMetricDescription {
        self.weight = Some(weight);
        self
    }

    pub fn weight(&self) -> Option<&::models::ServiceLoadMetricWeight> {
        self.weight.as_ref()
    }

    pub fn reset_weight(&mut self) {
        self.weight = None;
    }

    pub fn set_primary_default_load(&mut self, primary_default_load: i32) {
        self.primary_default_load = Some(primary_default_load);
    }

    pub fn with_primary_default_load(
        mut self,
        primary_default_load: i32,
    ) -> ServiceLoadMetricDescription {
        self.primary_default_load = Some(primary_default_load);
        self
    }

    pub fn primary_default_load(&self) -> Option<&i32> {
        self.primary_default_load.as_ref()
    }

    pub fn reset_primary_default_load(&mut self) {
        self.primary_default_load = None;
    }

    pub fn set_secondary_default_load(&mut self, secondary_default_load: i32) {
        self.secondary_default_load = Some(secondary_default_load);
    }

    pub fn with_secondary_default_load(
        mut self,
        secondary_default_load: i32,
    ) -> ServiceLoadMetricDescription {
        self.secondary_default_load = Some(secondary_default_load);
        self
    }

    pub fn secondary_default_load(&self) -> Option<&i32> {
        self.secondary_default_load.as_ref()
    }

    pub fn reset_secondary_default_load(&mut self) {
        self.secondary_default_load = None;
    }

    pub fn set_default_load(&mut self, default_load: i32) {
        self.default_load = Some(default_load);
    }

    pub fn with_default_load(
        mut self,
        default_load: i32,
    ) -> ServiceLoadMetricDescription {
        self.default_load = Some(default_load);
        self
    }

    pub fn default_load(&self) -> Option<&i32> {
        self.default_load.as_ref()
    }

    pub fn reset_default_load(&mut self) {
        self.default_load = None;
    }
}
