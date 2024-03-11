/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionLoadInformation : Represents load information for a partition, which contains the primary and secondary reported load metrics. In case there is no load reported, PartitionLoadInformation will contain the default load for the service of the partition. For default loads, LoadMetricReport's LastReportedUtc is set to 0.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionLoadInformation {
    /// Id of the partition.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// Array of load reports from the primary replica for this partition.
    #[serde(rename = "PrimaryLoadMetricReports")]
    primary_load_metric_reports: Option<Vec<::models::LoadMetricReport>>,
    /// Array of aggregated load reports from all secondary replicas for this partition. Array only contains the latest reported load for each metric.
    #[serde(rename = "SecondaryLoadMetricReports")]
    secondary_load_metric_reports: Option<Vec<::models::LoadMetricReport>>,
}

impl Default for PartitionLoadInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionLoadInformation {
    /// Represents load information for a partition, which contains the primary and secondary reported load metrics. In case there is no load reported, PartitionLoadInformation will contain the default load for the service of the partition. For default loads, LoadMetricReport's LastReportedUtc is set to 0.
    pub fn new() -> PartitionLoadInformation {
        PartitionLoadInformation {
            partition_id: None,
            primary_load_metric_reports: None,
            secondary_load_metric_reports: None,
        }
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> PartitionLoadInformation {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_primary_load_metric_reports(
        &mut self,
        primary_load_metric_reports: Vec<::models::LoadMetricReport>,
    ) {
        self.primary_load_metric_reports = Some(primary_load_metric_reports);
    }

    pub fn with_primary_load_metric_reports(
        mut self,
        primary_load_metric_reports: Vec<::models::LoadMetricReport>,
    ) -> PartitionLoadInformation {
        self.primary_load_metric_reports = Some(primary_load_metric_reports);
        self
    }

    pub fn primary_load_metric_reports(
        &self,
    ) -> Option<&Vec<::models::LoadMetricReport>> {
        self.primary_load_metric_reports.as_ref()
    }

    pub fn reset_primary_load_metric_reports(&mut self) {
        self.primary_load_metric_reports = None;
    }

    pub fn set_secondary_load_metric_reports(
        &mut self,
        secondary_load_metric_reports: Vec<::models::LoadMetricReport>,
    ) {
        self.secondary_load_metric_reports =
            Some(secondary_load_metric_reports);
    }

    pub fn with_secondary_load_metric_reports(
        mut self,
        secondary_load_metric_reports: Vec<::models::LoadMetricReport>,
    ) -> PartitionLoadInformation {
        self.secondary_load_metric_reports =
            Some(secondary_load_metric_reports);
        self
    }

    pub fn secondary_load_metric_reports(
        &self,
    ) -> Option<&Vec<::models::LoadMetricReport>> {
        self.secondary_load_metric_reports.as_ref()
    }

    pub fn reset_secondary_load_metric_reports(&mut self) {
        self.secondary_load_metric_reports = None;
    }
}
