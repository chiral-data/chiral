//! Run GMX Command
//! 

use serde::{Serialize, Deserialize};
use crate::traits::*;
use chiral_derive::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;


/// Input
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization)] 
pub struct Input {
    pub simulation_id: String,
    pub sub_command: String,
    pub arguments: Vec<String>,
    pub prompts: Vec<String>,
    pub files_dir: String, // files will be saved in directory files_dir/simulation/
    pub files_input: Vec<String>,
    pub files_output: Vec<String>
}

impl TraitInput for Input {
    fn default() -> Self {
        Self { 
            simulation_id: "lysozyme".to_string(),
            sub_command: "pdb2gmx".to_string(),
            arguments: ["-f", "1AKI_clean.pdb", "-o", "1AKI_processed.gro", "-water", "spce"].iter().map(|s| s.to_string()).collect(),
            prompts: ["15 0"].iter().map(|s| s.to_string()).collect(),
            files_dir: ".".to_string(),
            files_input: ["1AKI_clean.pdb"].iter().map(|s| s.to_string()).collect(),
            files_output: ["1AKI_processed.gro", "topol.top", "posre.itp"].iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TraitFileRequirements for Input {
    fn dir(&self) -> String {
        self.simulation_id.to_owned()
    }

    fn dir_full(&self) -> String {
        std::path::PathBuf::from(&self.files_dir).join(&self.simulation_id).to_string_lossy().to_string()
    }

    fn files_in(&self) -> Vec<String> {
        self.files_input.to_owned()
    }

    fn files_out(&self) -> Vec<String> {
         self.files_output.to_owned()
    }
}

/// Output
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization)]
pub struct Output {
    // #[pyo3(get)]
    pub success: bool,
    // #[pyo3(get)]
    pub stdout: String,
    // #[pyo3(get)]
    pub stderr: String
}

impl TraitOutput for Output {
    fn blank() -> Self { Self { success: false, stdout: "".to_string(), stderr: "".to_string() } }

    fn len(&self) -> usize { panic!("not applicable") }

    fn clear(&mut self) {
        self.success = false;
        self.stdout.clear();
        self.stderr.clear();
    }

    fn append(&mut self, other: &mut Self) {
        self.success = other.success;
        self.stdout += other.stdout.as_str();
        self.stderr += other.stderr.as_str();
    }
}

/// Report
#[add_report_fields]
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, ImplReport)]
pub struct Report {}

impl crate::traits::TraitReport for Report {
    fn print(&self) {
        unimplemented!("not implemented yet")
    }
}


///
/// Python Bindings
/// 
#[cfg_attr(feature = "python", pymethods)]
impl Input {
    #[cfg(feature = "python")]
    #[new]
    pub fn __new__(simulation_id: String, sub_command: String, arguments: Vec<String>, prompts: Vec<String>, files_dir: String, files_input: Vec<String>, files_output: Vec<String> ) -> Self {
        Self { simulation_id, sub_command, arguments, prompts, files_dir, files_input, files_output }
    }

    pub fn to_str(&self) -> String {
        self.ser_to()
    }
}

#[cfg_attr(feature = "python", pymethods)]
impl Output {
    #[cfg(feature = "python")]
    #[new]
    pub fn __new__(s: String) -> Self {
        Self::ser_from(&s)
    }
}

#[cfg(feature = "python")]
#[pymodule]
pub fn gromacs_gmx_command(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Input>()?;
    m.add_class::<Output>()?;
    Ok(())
}