//! Job Status
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq, Copy)]  
pub enum StatusLabel {
    Created,
    Processing,
    Completed,
    Cancelled,
    ErrorJobIDNotFound
}

impl std::fmt::Display for StatusLabel {
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

type CountTaskWaiting = super::DividendSize;
type CountTaskProcessing = super::DividendSize;
type CountTaskCompleted = super::DividendSize;
type ProgressTuple = (CountTaskWaiting, CountTaskProcessing, CountTaskCompleted);

/// Status
#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)] 
pub struct Status {
    label: StatusLabel,
    progress: Option<ProgressTuple>
}

impl Status {
    pub fn new(label: StatusLabel, progress: Option<(CountTaskWaiting, CountTaskProcessing, CountTaskCompleted)>) -> Self {
        Self { label, progress }
    }

    pub fn is_created(&self) -> bool { self.label == StatusLabel::Created }
    pub fn is_processing(&self) -> bool { self.label == StatusLabel::Processing }
    pub fn is_completed(&self) -> bool { self.label == StatusLabel::Completed }
    pub fn is_error_job_not_found(&self) -> bool { self.label == StatusLabel::ErrorJobIDNotFound }

    pub fn count_completed(&self) -> (usize, usize) {
        match self.progress {
            Some((w, p, c)) => (c, w + p + c), 
            None => (0, 0)
        }
    }
    
    pub fn get_progress(&self) -> f32 {
        match self.progress {
            Some((w, p, c)) => {
                if w + p + c == 0 {
                    0.0
                } else {
                    c as f32 / (w + p + c) as f32
                }
            },
            None => 0.0
        }
    }

    pub fn assign_task(&mut self, dividends: usize) {
        match &self.label {
            StatusLabel::Created => {
                self.label = StatusLabel::Processing;
                self.progress = Some((dividends - 1, 1, 0));
            },
            StatusLabel::Processing => {
                let (w, p, c) = self.progress.expect("job in process should not have null progress");
                self.progress = Some((w - 1, p + 1, c))
            },
            _ => {
                crate::logging::warn("job status: should not reach here");
            }
        }
    }

    pub fn complete_task(&mut self, dividends: usize) {
        match &self.label {
            StatusLabel::Processing => {
                let (w, p, c) = self.progress.expect("job in process should not have null progress");
                self.progress = Some((w, p - 1, c + 1));
                if c + 1 == dividends {
                    self.label = StatusLabel::Completed;
                } 
            },
            _ => {
                crate::logging::warn("job status: should not reach here");
            }
        };
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let progress_str = match self.progress {
            Some(p) => format!("waiting: {}, processing: {}, completed: {}", p.0, p.1, p.2),
            None => format!("")
        };
        write!(f, "{}", format!("{} {}", self.label, progress_str))
    }
}