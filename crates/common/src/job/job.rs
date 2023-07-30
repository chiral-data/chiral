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
    Completed,
    Cancelled,
    Unknown,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "CREATED"),
            Self::Processing => write!(f, "PROCESSING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
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
    #[serde(with = "ts_milliseconds")]
    pub time_submitted: chrono::DateTime<chrono::Utc>,
    #[serde(with = "ts_milliseconds")]
    pub time_start: chrono::DateTime<chrono::Utc>,
    pub duration: Option<std::time::Duration>,
    pub cost: crate::types::CreditPoints,
}

impl Job {
    pub fn new(requirement: super::Requirement, divisor: usize) -> Self {
        let progress = match requirement.get_opk().computation_kind() {
            crate::kinds::ComputationKind::Simulation => Progress::ByPercentage(0.0),
            crate::kinds::ComputationKind::DataProcessing =>  Progress::ByBlocks(divisor, 0, 0)
        };

        Self {
            id: generate_id(),
            requirement,
            status: Status::Created,
            progress,
            outputs: vec![None; divisor],
            time_submitted: chrono::Utc::now(),
            time_start: chrono::Utc::now(),
            duration: None,
            cost: 0.0
        }
    }

    pub fn assign_task(&mut self)  {
        match self.progress {
            Progress::ByBlocks(w, p, c) => self.progress = Progress::ByBlocks(w - 1, p + 1 , c),
            Progress::ByPercentage(_) => () 
        }

        if self.status == Status::Created {
            self.status = Status::Processing;
            self.time_start = chrono::Utc::now()
        }
    }

    pub fn reset_task(&mut self) {
        match self.progress {
            Progress::ByBlocks(w, p, c) => self.progress = Progress::ByBlocks(w + 1, p - 1 , c),
            Progress::ByPercentage(_) => () 
        }
    }

    pub fn complete_task(&mut self, index: usize, output: Option<SerializedFormat>, cost: &crate::types::CreditPoints) {
        match self.progress {
            Progress::ByBlocks(w, p, c) => self.progress = Progress::ByBlocks(w, p - 1 , c + 1),
            Progress::ByPercentage(_) => () 
        }

        self.outputs[index] = output; 
        if self.outputs.iter().position(|o| o.is_none()).is_none() {
            self.status = Status::Completed;
            self.duration = match (chrono::Utc::now() - self.time_start).to_std() {
                Ok(d) => Some(d),
                Err(e) => {
                    crate::logging::error(format!("chrono::Duration to std::time::Duration conversion error: {e}").as_str());
                    Some(std::time::Duration::from_secs(0))
                }
            }
        }

        self.cost += cost;
    }


    // pub fn start(&mut self) {
    //     self.status_label = super::StatusLabel::Processing;
    //     self.time_start = chrono::Utc::now()
    // }

    // pub fn complete(&mut self) {
    //     self.status_label = super::StatusLabel::Completed;
    //     self.duration = Some((chrono::Utc::now() - self.time_start).to_std().unwrap());
    // }

    // pub fn report_done(&mut self) {
    //     self.report_ready = true;
    // }

    // pub fn cancel(&mut self) {
    //     self.status_label = super::StatusLabel::Cancelled;
    // }

    // pub fn add_duration_prep(&mut self, d: &std::time::Duration) {
    //     self.duration_prep = match self.duration_prep {
    //         Some(dp) => Some(dp + *d),
    //         None => Some(*d)
    //     };
    // }

    // pub fn get_id(&self) -> &super::ID { &self.id }
    // pub fn get_req(&self) -> &super::Requirement { &self.req }
    // pub fn get_input(&self) -> &SerializedFormat { self.req.get_ji() }
    // pub fn get_opk(&self) -> &crate::kinds::Operator { self.req.get_opk() }
    // pub fn get_dsk(&self) -> &crate::kinds::Dataset { self.req.get_dsk() }

    // pub fn is_status(&self, comp_status: super::StatusLabel) -> bool {
    //     self.status_label == comp_status
    // }

    // pub fn is_report_ready(&self) -> bool { 
    //     self.report_ready
    // }

    // pub fn set_status(&mut self, new_status: super::StatusLabel) {
    //     self.status_label = new_status;
    // }

    fn print_time_properties(&self) -> String {
        format!("{}\t{:.2}", self.time_start.format("%Y-%m-%d %H:%M:%S").to_string(), self.duration.map_or(0.0, |d| d.as_secs_f32()))
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} {}\t\t{:20}\t{:15}\t{}", self.id, self.status, self.requirement.get_opk().to_string(), self.requirement.get_dsk().to_string(), self.print_time_properties()).fmt(f)
    }
}