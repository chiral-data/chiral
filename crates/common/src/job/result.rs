//! Job Result
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

/// Task Result: a job is divided into several tasks
#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq)] 
pub struct Result {
    pub outputs: Vec<SerializedFormat>,
    pub error: Option<String>
}