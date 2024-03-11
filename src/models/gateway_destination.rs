/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GatewayDestination : Describes destination endpoint for routing traffic.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayDestination {
    /// Name of the service fabric Mesh application.
    #[serde(rename = "applicationName")]
    application_name: String,
    /// service that contains the endpoint.
    #[serde(rename = "serviceName")]
    service_name: String,
    /// name of the endpoint in the service.
    #[serde(rename = "endpointName")]
    endpoint_name: String,
}

impl GatewayDestination {
    /// Describes destination endpoint for routing traffic.
    pub fn new(
        application_name: String,
        service_name: String,
        endpoint_name: String,
    ) -> GatewayDestination {
        GatewayDestination {
            application_name,
            service_name,
            endpoint_name,
        }
    }

    pub fn set_application_name(&mut self, application_name: String) {
        self.application_name = application_name;
    }

    pub fn with_application_name(
        mut self,
        application_name: String,
    ) -> GatewayDestination {
        self.application_name = application_name;
        self
    }

    pub fn application_name(&self) -> &String {
        &self.application_name
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> GatewayDestination {
        self.service_name = service_name;
        self
    }

    pub fn service_name(&self) -> &String {
        &self.service_name
    }

    pub fn set_endpoint_name(&mut self, endpoint_name: String) {
        self.endpoint_name = endpoint_name;
    }

    pub fn with_endpoint_name(
        mut self,
        endpoint_name: String,
    ) -> GatewayDestination {
        self.endpoint_name = endpoint_name;
        self
    }

    pub fn endpoint_name(&self) -> &String {
        &self.endpoint_name
    }
}
