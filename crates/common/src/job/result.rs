//! Job Result
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

/// Task Result: a job is divided into several tasks
#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)] 
pub struct TaskResult {
    output: SerializedFormat,
    error: Option<String>,
    duration_init: std::time::Duration,
    duration_whole: Option<std::time::Duration>
}

impl TaskResult {
    pub fn new(output: SerializedFormat, error: Option<String>, duration_init: std::time::Duration) -> Self {
        Self { output, error, duration_init, duration_whole: None }
    }

    pub fn set_whole_duration(&mut self,  duration_whole: std::time::Duration) {
        self.duration_whole = Some(duration_whole);
    }

    pub fn get_output(&self) -> &SerializedFormat {
        &self.output
    }
}

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)] 
pub struct Result {
    req: super::requirement::Requirement,
    results: Vec<Option<TaskResult>>
}

impl Result {
    pub fn null_string() -> String { "".to_string() }

    pub fn new(req: super::requirement::Requirement, dividends: super::DividendSize) -> Self {
        Self {
            req,
            results: vec![None; dividends]
        }
    }

    pub fn get_req(&self) -> &super::requirement::Requirement {
        &self.req
    }

    pub fn set(&mut self, idx: super::DividendSize, result: TaskResult) {
        self.results[idx] = Some(result);
    }

    pub fn count_completed_tasks(&self) -> usize {
        self.results.iter().filter(|r| r.is_some()).count()
    }

    pub fn save_report(&self, job_id: super::ID, filepath: &std::path::PathBuf) -> std::io::Result<u64>  {
        let opk = self.req.get_opk();
        let output_sers = self.results.iter()
            .filter(|op_tr| op_tr.is_some())
            .map(|some_tr| some_tr.as_ref().unwrap().get_output().to_owned())
            .collect();

        opk.report_save(job_id, self.req.get_dsk().to_owned(), self.get_req().get_ji(), &output_sers, filepath) 
    }

    pub fn get_outputs(&self) -> Vec<SerializedFormat> {
        self.results.iter()
            .filter(|r| r.is_some())
            .map(|r| r.to_owned().unwrap().get_output().to_owned())
            .collect()
    }
}