/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeHealthPolicyMap : Defines a ServiceTypeHealthPolicy per service type name.  The entries in the map replace the default service type health policy for each specified service type. For example, in an application that contains both a stateless gateway service type and a stateful engine service type, the health policies for the stateless and stateful services can be configured differently. With policy per service type, there's more granular control of the health of the service.  If no policy is specified for a service type name, the DefaultServiceTypeHealthPolicy is used for evaluation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeHealthPolicyMap {}

impl Default for ServiceTypeHealthPolicyMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeHealthPolicyMap {
    /// Defines a ServiceTypeHealthPolicy per service type name.  The entries in the map replace the default service type health policy for each specified service type. For example, in an application that contains both a stateless gateway service type and a stateful engine service type, the health policies for the stateless and stateful services can be configured differently. With policy per service type, there's more granular control of the health of the service.  If no policy is specified for a service type name, the DefaultServiceTypeHealthPolicy is used for evaluation.
    pub fn new() -> ServiceTypeHealthPolicyMap {
        ServiceTypeHealthPolicyMap {}
    }
}
