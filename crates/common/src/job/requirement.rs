//! Requirement of a Job
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;
use crate::traits::TraitInput;

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

    pub fn requirement_gromacs_for_tests() -> Self {
        Self {
            ji: crate::apps::chem::gromacs::gmx_command::Input::default().ser_to(),
            opk: crate::kinds::Operator::GromacsRunGMXCommand,
            dsk: crate::kinds::Dataset::Empty,
        }
    }

    pub fn requirement_openbabel_for_tests() -> Self {
        Self {
            ji: "c1cccc1N=O".to_string(),
            opk: crate::kinds::Operator::OpenBabelSSMatching,
            dsk: crate::kinds::Dataset::TestChembl,
        }
    }

    pub fn requirement_recgen_for_tests() -> Self {
        Self {
            ji: crate::apps::chem::recgen::build::Input::default().ser_to(),
            opk: crate::kinds::Operator::ReCGenBuild,
            dsk: crate::kinds::Dataset::Empty,
        }
    }
}