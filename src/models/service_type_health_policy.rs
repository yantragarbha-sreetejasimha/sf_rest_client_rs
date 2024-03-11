/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeHealthPolicy : Represents the health policy used to evaluate the health of services belonging to a service type.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeHealthPolicy {
    /// The maximum allowed percentage of unhealthy partitions per service. Allowed values are Byte values from zero to 100  The percentage represents the maximum tolerated percentage of partitions that can be unhealthy before the service is considered in error. If the percentage is respected but there is at least one unhealthy partition, the health is evaluated as Warning. The percentage is calculated by dividing the number of unhealthy partitions over the total number of partitions in the service. The computation rounds up to tolerate one failure on small numbers of partitions. Default percentage is zero.
    #[serde(rename = "MaxPercentUnhealthyPartitionsPerService")]
    max_percent_unhealthy_partitions_per_service: Option<i32>,
    /// The maximum allowed percentage of unhealthy replicas per partition. Allowed values are Byte values from zero to 100.  The percentage represents the maximum tolerated percentage of replicas that can be unhealthy before the partition is considered in error. If the percentage is respected but there is at least one unhealthy replica, the health is evaluated as Warning. The percentage is calculated by dividing the number of unhealthy replicas over the total number of replicas in the partition. The computation rounds up to tolerate one failure on small numbers of replicas. Default percentage is zero.
    #[serde(rename = "MaxPercentUnhealthyReplicasPerPartition")]
    max_percent_unhealthy_replicas_per_partition: Option<i32>,
    /// The maximum allowed percentage of unhealthy services. Allowed values are Byte values from zero to 100.  The percentage represents the maximum tolerated percentage of services that can be unhealthy before the application is considered in error. If the percentage is respected but there is at least one unhealthy service, the health is evaluated as Warning. This is calculated by dividing the number of unhealthy services of the specific service type over the total number of services of the specific service type. The computation rounds up to tolerate one failure on small numbers of services. Default percentage is zero.
    #[serde(rename = "MaxPercentUnhealthyServices")]
    max_percent_unhealthy_services: Option<i32>,
}

impl Default for ServiceTypeHealthPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeHealthPolicy {
    /// Represents the health policy used to evaluate the health of services belonging to a service type.
    pub fn new() -> ServiceTypeHealthPolicy {
        ServiceTypeHealthPolicy {
            max_percent_unhealthy_partitions_per_service: None,
            max_percent_unhealthy_replicas_per_partition: None,
            max_percent_unhealthy_services: None,
        }
    }

    pub fn set_max_percent_unhealthy_partitions_per_service(
        &mut self,
        max_percent_unhealthy_partitions_per_service: i32,
    ) {
        self.max_percent_unhealthy_partitions_per_service =
            Some(max_percent_unhealthy_partitions_per_service);
    }

    pub fn with_max_percent_unhealthy_partitions_per_service(
        mut self,
        max_percent_unhealthy_partitions_per_service: i32,
    ) -> ServiceTypeHealthPolicy {
        self.max_percent_unhealthy_partitions_per_service =
            Some(max_percent_unhealthy_partitions_per_service);
        self
    }

    pub fn max_percent_unhealthy_partitions_per_service(&self) -> Option<&i32> {
        self.max_percent_unhealthy_partitions_per_service.as_ref()
    }

    pub fn reset_max_percent_unhealthy_partitions_per_service(&mut self) {
        self.max_percent_unhealthy_partitions_per_service = None;
    }

    pub fn set_max_percent_unhealthy_replicas_per_partition(
        &mut self,
        max_percent_unhealthy_replicas_per_partition: i32,
    ) {
        self.max_percent_unhealthy_replicas_per_partition =
            Some(max_percent_unhealthy_replicas_per_partition);
    }

    pub fn with_max_percent_unhealthy_replicas_per_partition(
        mut self,
        max_percent_unhealthy_replicas_per_partition: i32,
    ) -> ServiceTypeHealthPolicy {
        self.max_percent_unhealthy_replicas_per_partition =
            Some(max_percent_unhealthy_replicas_per_partition);
        self
    }

    pub fn max_percent_unhealthy_replicas_per_partition(&self) -> Option<&i32> {
        self.max_percent_unhealthy_replicas_per_partition.as_ref()
    }

    pub fn reset_max_percent_unhealthy_replicas_per_partition(&mut self) {
        self.max_percent_unhealthy_replicas_per_partition = None;
    }

    pub fn set_max_percent_unhealthy_services(
        &mut self,
        max_percent_unhealthy_services: i32,
    ) {
        self.max_percent_unhealthy_services =
            Some(max_percent_unhealthy_services);
    }

    pub fn with_max_percent_unhealthy_services(
        mut self,
        max_percent_unhealthy_services: i32,
    ) -> ServiceTypeHealthPolicy {
        self.max_percent_unhealthy_services =
            Some(max_percent_unhealthy_services);
        self
    }

    pub fn max_percent_unhealthy_services(&self) -> Option<&i32> {
        self.max_percent_unhealthy_services.as_ref()
    }

    pub fn reset_max_percent_unhealthy_services(&mut self) {
        self.max_percent_unhealthy_services = None;
    }
}
