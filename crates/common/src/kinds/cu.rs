//! Computing Unit Kind
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, PartialEq, Eq, Hash)]  
pub struct Kind {
    opk: crate::kinds::Operator,
    dsk: crate::kinds::Dataset
}

impl Kind {
    pub fn new(opk: crate::kinds::Operator, dsk: crate::kinds::Dataset) -> Self {
        Self { opk, dsk }
    }

    pub fn get_opk(&self) -> &crate::kinds::Operator { &self.opk }
    pub fn get_dsk(&self) -> &crate::kinds::Dataset { &self.dsk }
}

impl std::default::Default for Kind {
    fn default() -> Self {
        Self::new(crate::kinds::Operator::OpenBabelSSMatching, crate::kinds::Dataset::TestChembl)
    }
}