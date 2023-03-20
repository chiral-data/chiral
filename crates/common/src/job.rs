//! Common for Job
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;
use chrono::serde::ts_milliseconds;

pub type ID = String;
pub type DividendSize = usize;
pub type DividendIndex = (DividendSize, DividendSize);

fn generate_id() -> ID {
    crate::utils::generate_id(32)
}

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq, Hash)]  
pub struct Requirement {
    ji: SerializedFormat, 
    opk: crate::kinds::Operator,
    dsk: crate::kinds::Dataset,
}

impl Requirement {
    pub fn new(ji: SerializedFormat, opk: crate::kinds::Operator, dsk: crate::kinds::Dataset) -> Self {
        Self { ji, opk, dsk }
    }

    pub fn get_ji(&self) -> &SerializedFormat { &self.ji }
    pub fn get_opk(&self) -> &crate::kinds::Operator { &self.opk }
    pub fn get_dsk(&self) -> &crate::kinds::Dataset { &self.dsk }
    pub fn generate_cuk(&self) -> crate::kinds::ComputingUnit { crate::kinds::ComputingUnit::new(self.opk.to_owned(), self.dsk.to_owned()) }
}

impl std::default::Default for Requirement {
    /// default job::Requirement: Substructure Matching on TestChembl
    fn default() -> Self {
        Self {
            ji: "c1cccc1N=O".to_string(),
            opk: crate::kinds::Operator::OpenBabelSSMatching,
            dsk: crate::kinds::Dataset::TestChembl
        }
    }
}

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq, Copy)]  
pub enum Status {
    Created,
    Processing,
    Completed,
    Cancelled,
    ErrorJobIDNotFound
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "CREATED"),
            Self::Processing => write!(f, "PROCESSING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::ErrorJobIDNotFound => write!(f, "ERROR_ID_NOT_FOUND")
        }
    }
}


#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)]  
pub struct Job {
    id: ID,
    req: Requirement,
    status: Status,
    report_ready: bool, // report ready in local filesystem
    #[serde(with = "ts_milliseconds")]
    time_start: chrono::DateTime<chrono::Utc>,
    duration_prep: Option<std::time::Duration>, // time for data preparation
    duration: Option<std::time::Duration>
}

impl Job {
    pub fn new(req: Requirement) -> Self {
        let id = generate_id();
        Self { id, req, status: Status::Created, report_ready: false, time_start: chrono::Utc::now(), duration_prep: None, duration: None }
    }

    pub fn set_id(&mut self, id: ID) {
        self.id = id;
    }

    pub fn start(&mut self) {
        self.status = Status::Processing;
        self.time_start = chrono::Utc::now()
    }

    pub fn complete(&mut self) {
        self.status = Status::Completed;
        self.duration = Some((chrono::Utc::now() - self.time_start).to_std().unwrap());
    }

    pub fn report_done(&mut self) {
        self.report_ready = true;
    }

    pub fn cancel(&mut self) {
        self.status = Status::Cancelled;
    }

    pub fn add_duration_prep(&mut self, d: &std::time::Duration) {
        self.duration_prep = match self.duration_prep {
            Some(dp) => Some(dp + *d),
            None => Some(*d)
        };
    }

    pub fn get_id(&self) -> &ID { &self.id }
    pub fn get_req(&self) -> &Requirement { &self.req }
    pub fn get_input(&self) -> &SerializedFormat { self.req.get_ji() }
    pub fn get_opk(&self) -> &crate::kinds::Operator { self.req.get_opk() }
    pub fn get_dsk(&self) -> &crate::kinds::Dataset { self.req.get_dsk() }

    pub fn is_status(&self, comp_status: Status) -> bool {
        self.status == comp_status
    }

    pub fn is_report_ready(&self) -> bool { 
        self.report_ready
    }

    pub fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    fn print_time_properties(&self) -> String {
        format!("{}\t{:.2}\t{:.2}", self.time_start.format("%Y-%m-%d %H:%M:%S").to_string(), self.duration_prep.map_or(0.0, |d| d.as_secs_f32()), self.duration.map_or(0.0, |d| d.as_secs_f32()))
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} {}\t\t{:20}\t{:15}\t{}", self.id, self.status, self.req.get_opk().to_string(), self.req.get_dsk().to_string(), self.print_time_properties()).fmt(f)
    }
}

pub type Jobs = std::collections::HashMap<ID, Job>;


