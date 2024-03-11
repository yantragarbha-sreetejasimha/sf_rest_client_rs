/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServicePlacementInvalidDomainPolicyDescription : Describes the policy to be used for placement of a Service Fabric service where a particular fault or upgrade domain should not be used for placement of the instances or replicas of that service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicePlacementInvalidDomainPolicyDescription {
    /// The type of placement policy for a service fabric service. Following are the possible values.
    #[serde(rename = "Type")]
    _type: ::models::ServicePlacementPolicyType,
    /// The name of the domain that should not be used for placement.
    #[serde(rename = "DomainName")]
    domain_name: Option<String>,
}

impl ServicePlacementInvalidDomainPolicyDescription {
    /// Describes the policy to be used for placement of a Service Fabric service where a particular fault or upgrade domain should not be used for placement of the instances or replicas of that service.
    pub fn new(
        _type: ::models::ServicePlacementPolicyType,
    ) -> ServicePlacementInvalidDomainPolicyDescription {
        ServicePlacementInvalidDomainPolicyDescription {
            _type,
            domain_name: None,
        }
    }

    pub fn set_type(&mut self, _type: ::models::ServicePlacementPolicyType) {
        self._type = _type;
    }

    pub fn with_type(
        mut self,
        _type: ::models::ServicePlacementPolicyType,
    ) -> ServicePlacementInvalidDomainPolicyDescription {
        self._type = _type;
        self
    }

    pub fn _type(&self) -> &::models::ServicePlacementPolicyType {
        &self._type
    }

    pub fn set_domain_name(&mut self, domain_name: String) {
        self.domain_name = Some(domain_name);
    }

    pub fn with_domain_name(
        mut self,
        domain_name: String,
    ) -> ServicePlacementInvalidDomainPolicyDescription {
        self.domain_name = Some(domain_name);
        self
    }

    pub fn domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
    }

    pub fn reset_domain_name(&mut self) {
        self.domain_name = None;
    }
}
