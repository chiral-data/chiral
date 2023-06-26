//! Common for Job
//! 

use chrono::serde::ts_milliseconds;
use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

fn generate_id() -> super::ID {
    crate::utils::generate_id(32)
}

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)]  
pub struct Job {
    id: super::ID,
    req: super::Requirement,
    status_label: super::StatusLabel,
    report_ready: bool, // report ready in local filesystem
    #[serde(with = "ts_milliseconds")]
    time_start: chrono::DateTime<chrono::Utc>,
    duration_prep: Option<std::time::Duration>, // time for data preparation
    duration: Option<std::time::Duration>
}

impl Job {
    pub fn new(req: super::Requirement) -> Self {
        let id = generate_id();
        Self { id, req, status_label: super::StatusLabel::Created, report_ready: false, time_start: chrono::Utc::now(), duration_prep: None, duration: None }
    }

    pub fn set_id(&mut self, id: super::ID) {
        self.id = id;
    }

    pub fn start(&mut self) {
        self.status_label = super::StatusLabel::Processing;
        self.time_start = chrono::Utc::now()
    }

    pub fn complete(&mut self) {
        self.status_label = super::StatusLabel::Completed;
        self.duration = Some((chrono::Utc::now() - self.time_start).to_std().unwrap());
    }

    pub fn report_done(&mut self) {
        self.report_ready = true;
    }

    pub fn cancel(&mut self) {
        self.status_label = super::StatusLabel::Cancelled;
    }

    pub fn add_duration_prep(&mut self, d: &std::time::Duration) {
        self.duration_prep = match self.duration_prep {
            Some(dp) => Some(dp + *d),
            None => Some(*d)
        };
    }

    pub fn get_id(&self) -> &super::ID { &self.id }
    pub fn get_req(&self) -> &super::Requirement { &self.req }
    pub fn get_input(&self) -> &SerializedFormat { self.req.get_ji() }
    pub fn get_opk(&self) -> &crate::kinds::Operator { self.req.get_opk() }
    pub fn get_dsk(&self) -> &crate::kinds::Dataset { self.req.get_dsk() }

    pub fn is_status(&self, comp_status: super::StatusLabel) -> bool {
        self.status_label == comp_status
    }

    pub fn is_report_ready(&self) -> bool { 
        self.report_ready
    }

    pub fn set_status(&mut self, new_status: super::StatusLabel) {
        self.status_label = new_status;
    }

    fn print_time_properties(&self) -> String {
        format!("{}\t{:.2}\t{:.2}", self.time_start.format("%Y-%m-%d %H:%M:%S").to_string(), self.duration_prep.map_or(0.0, |d| d.as_secs_f32()), self.duration.map_or(0.0, |d| d.as_secs_f32()))
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} {}\t\t{:20}\t{:15}\t{}", self.id, self.status_label, self.req.get_opk().to_string(), self.req.get_dsk().to_string(), self.print_time_properties()).fmt(f)
    }
}