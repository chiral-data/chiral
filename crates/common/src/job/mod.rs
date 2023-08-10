mod requirement;
// mod status;
mod result;
mod job;

pub type DividendSize = usize;
pub type DividendIndex = (DividendSize, DividendSize);
pub use requirement::Requirement;
pub use result::Result;
pub use job::Status;
pub use job::Job;
pub type Jobs = std::collections::HashMap<crate::types::JobID, Job>;

//
//  Python Bindings
//

use std::str::FromStr;
use crate::traits::Serialization;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyfunction)]
pub fn create_requirement(ji_str: String, opk_str: String, dsk_str: String) -> String {
    let opk = crate::kinds::Operator::from_str(&opk_str).unwrap();
    let dsk = crate::kinds::Dataset::from_str(&dsk_str).unwrap();
    Requirement::new(ji_str, opk, dsk).ser_to()
}

#[cfg(feature = "python")]
#[pymodule]
pub fn job_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_requirement, m)?)?;
    Ok(())
}