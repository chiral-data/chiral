//! Common for Job
//! 

use chrono::serde::ts_milliseconds;
use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq, Copy)]  
pub enum Status {
    Created,
    Processing,
    CompletedSuccess,
    CompletedError,
    Cancelled,
    Unknown,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "CREATED"),
            Self::Processing => write!(f, "PROCESSING"),
            Self::CompletedSuccess => write!(f, "COMPLETED SUCCESS"),
            Self::CompletedError => write!(f, "COMPLETED ERROR"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Status {
    pub fn is_completed(&self) -> bool {
        *self == Self::CompletedSuccess || *self == Self::CompletedError
    }
}

#[derive(Debug, Serialize, Deserialize, Serialization, Clone, PartialEq)]
pub enum Progress {
    ByPercentage(f32),
    ByBlocks(usize, usize, usize), // waiting, processing, completed
}

fn generate_id() -> crate::types::JobID {
    crate::utils::generate_id(32)
}

// #[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)]  
// pub struct Job {
//     id: super::ID,
//     req: super::Requirement,
//     status_label: super::StatusLabel,
//     report_ready: bool, // report ready in local filesystem
//     #[serde(with = "ts_milliseconds")]
//     time_start: chrono::DateTime<chrono::Utc>,
//     duration_prep: Option<std::time::Duration>, // time for data preparation
//     duration: Option<std::time::Duration>
// }

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq)]   
pub struct Job {
    pub id: crate::types::JobID,
    pub requirement: super::Requirement,
    pub status: Status,
    pub progress: Progress,
    pub outputs: Vec<Option<SerializedFormat>>,
    pub error: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub time_submitted: chrono::DateTime<chrono::Utc>,
    #[serde(with = "ts_milliseconds")]
    pub time_started: chrono::DateTime<chrono::Utc>,
    #[serde(with = "ts_milliseconds")]
    pub time_completed: chrono::DateTime<chrono::Utc>,
    pub cost: crate::types::CreditPoints,
}

impl Job {
    pub fn new(requirement: super::Requirement, divisor: usize) -> Self {
        let progress = match requirement.get_opk().computation_kind() {
            crate::kinds::ComputationKind::SingleMachineCPU | crate::kinds::ComputationKind::SingleMachineGPU => Progress::ByPercentage(0.0),
            crate::kinds::ComputationKind::MultipleMachines =>  Progress::ByBlocks(divisor, 0, 0)
        };

        Self {
            id: generate_id(),
            requirement,
            status: Status::Created,
            progress,
            outputs: vec![None; divisor],
            error: None,
            time_submitted: chrono::DateTime::<chrono::Utc>::MIN_UTC,
            time_started: chrono::DateTime::<chrono::Utc>::MIN_UTC,
            time_completed: chrono::DateTime::<chrono::Utc>::MIN_UTC,
            cost: 0.0
        }
    }

    pub fn submit(&mut self) {
        self.time_submitted = chrono::Utc::now();
    }

    pub fn assign_task(&mut self)  {
        match self.progress {
            Progress::ByBlocks(w, p, c) => self.progress = Progress::ByBlocks(w - 1, p + 1 , c),
            Progress::ByPercentage(_) => () 
        }

        if self.status == Status::Created {
            self.status = Status::Processing;
            self.time_started = chrono::Utc::now()
        }
    }

    pub fn reset_task(&mut self) {
        match self.progress {
            Progress::ByBlocks(w, p, c) => self.progress = Progress::ByBlocks(w + 1, p - 1 , c),
            Progress::ByPercentage(_) => () 
        }
    }

    pub fn complete_task_with_success(&mut self, index: usize, output: Option<SerializedFormat>, cost: &crate::types::CreditPoints) {
        match self.progress {
            Progress::ByBlocks(w, p, c) => self.progress = Progress::ByBlocks(w, p - 1 , c + 1),
            Progress::ByPercentage(_) => () 
        }

        // self.duration += match (chrono::Utc::now() - self.time_start).to_std() {
        //     Ok(d) => d,
        //     Err(e) => {
        //         crate::logging::error(format!("chrono::Duration to std::time::Duration conversion error: {e}").as_str());
        //         std::time::Duration::ZERO
        //     }
        // }

        self.outputs[index] = output; 
        if self.outputs.iter().position(|o| o.is_none()).is_none() {
            self.status = Status::CompletedSuccess;
            self.time_completed = chrono::Utc::now();
        }

        self.cost += cost;
    }

    pub fn complete_task_with_error(&mut self, error: String, cost: &crate::types::CreditPoints) {
        self.error = Some(error);
        self.status = Status::CompletedError;
        self.time_completed = chrono::Utc::now();
        self.cost += cost;
    }

    pub fn get_result(&self) -> crate::job::Result {
        if self.status.is_completed() {
            match &self.error {
                Some(error) => crate::job::Result { outputs: vec![], error: Some(error.to_string()) },
                None => {
                    let outputs = self.outputs.iter()
                        .filter_map(|o| o.to_owned())
                        .collect();
                    crate::job::Result { outputs, error: None }
                }
            }
        } else {
            crate::job::Result { outputs: vec![], error: None }
        }
    }

    fn print_time_properties(&self) -> String {
        format!("{}\t{}", self.time_started.format("%Y-%m-%d %H:%M:%S").to_string(), self.time_completed.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} {}\t\t{:20}\t{:15}\t{}", self.id, self.status, self.requirement.get_opk().to_string(), self.requirement.get_dsk().to_string(), self.print_time_properties()).fmt(f)
    }
}