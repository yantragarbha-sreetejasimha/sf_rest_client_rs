/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosSchedule : Defines the schedule used by Chaos.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosSchedule {
    /// The date and time Chaos will start using this schedule.
    #[serde(rename = "StartDate")]
    start_date: Option<String>,
    /// The date and time Chaos will continue to use this schedule until.
    #[serde(rename = "ExpiryDate")]
    expiry_date: Option<String>,
    /// A mapping of string names to Chaos Parameters to be referenced by Chaos Schedule Jobs.
    #[serde(rename = "ChaosParametersDictionary")]
    chaos_parameters_dictionary:
        Option<Vec<::models::ChaosParametersDictionaryItem>>,
    /// A list of all Chaos Schedule Jobs that will be automated by the schedule.
    #[serde(rename = "Jobs")]
    jobs: Option<Vec<::models::ChaosScheduleJob>>,
}

impl Default for ChaosSchedule {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosSchedule {
    /// Defines the schedule used by Chaos.
    pub fn new() -> ChaosSchedule {
        ChaosSchedule {
            start_date: None,
            expiry_date: None,
            chaos_parameters_dictionary: None,
            jobs: None,
        }
    }

    pub fn set_start_date(&mut self, start_date: String) {
        self.start_date = Some(start_date);
    }

    pub fn with_start_date(mut self, start_date: String) -> ChaosSchedule {
        self.start_date = Some(start_date);
        self
    }

    pub fn start_date(&self) -> Option<&String> {
        self.start_date.as_ref()
    }

    pub fn reset_start_date(&mut self) {
        self.start_date = None;
    }

    pub fn set_expiry_date(&mut self, expiry_date: String) {
        self.expiry_date = Some(expiry_date);
    }

    pub fn with_expiry_date(mut self, expiry_date: String) -> ChaosSchedule {
        self.expiry_date = Some(expiry_date);
        self
    }

    pub fn expiry_date(&self) -> Option<&String> {
        self.expiry_date.as_ref()
    }

    pub fn reset_expiry_date(&mut self) {
        self.expiry_date = None;
    }

    pub fn set_chaos_parameters_dictionary(
        &mut self,
        chaos_parameters_dictionary: Vec<
            ::models::ChaosParametersDictionaryItem,
        >,
    ) {
        self.chaos_parameters_dictionary = Some(chaos_parameters_dictionary);
    }

    pub fn with_chaos_parameters_dictionary(
        mut self,
        chaos_parameters_dictionary: Vec<
            ::models::ChaosParametersDictionaryItem,
        >,
    ) -> ChaosSchedule {
        self.chaos_parameters_dictionary = Some(chaos_parameters_dictionary);
        self
    }

    pub fn chaos_parameters_dictionary(
        &self,
    ) -> Option<&Vec<::models::ChaosParametersDictionaryItem>> {
        self.chaos_parameters_dictionary.as_ref()
    }

    pub fn reset_chaos_parameters_dictionary(&mut self) {
        self.chaos_parameters_dictionary = None;
    }

    pub fn set_jobs(&mut self, jobs: Vec<::models::ChaosScheduleJob>) {
        self.jobs = Some(jobs);
    }

    pub fn with_jobs(
        mut self,
        jobs: Vec<::models::ChaosScheduleJob>,
    ) -> ChaosSchedule {
        self.jobs = Some(jobs);
        self
    }

    pub fn jobs(&self) -> Option<&Vec<::models::ChaosScheduleJob>> {
        self.jobs.as_ref()
    }

    pub fn reset_jobs(&mut self) {
        self.jobs = None;
    }
}