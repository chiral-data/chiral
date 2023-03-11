//! OpenBabel Substructure Match
//! 

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive_new::Serialization;

/// Input
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization)] 
pub struct  Input {
    pub smarts: crate::app::chem::types::SMARTS
}

impl std::default::Default for Input {
    fn default() -> Self {
        Self { smarts: crate::app::chem::types::SMILES::from("c1ccccc1N=O") }
    }
}

/// Output
pub type MatchResult = Vec<Vec<i32>>;

#[derive(PartialEq, Debug, Serialize, Deserialize, Serialization, Clone)]
pub struct Output {
    pub results: Vec<(MatchResult, crate::data::types::EntryID)>
}

impl crate::traits::TraitOutput for Output {
    fn blank() -> Self { Self { results: vec![] } }
     
    fn len(&self) -> usize { self.results.len() }

    fn clear(&mut self) {
        self.results.clear();
    }

    fn append(&mut self, other: &mut Self) {
        self.results.append(&mut other.results);
    }
}

/// Report
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization)]
pub struct Report {
    pub input: Input,
    pub dsk: crate::kinds::Dataset,
    pub output: Output
}

impl crate::traits::TraitReport for Report {
    fn extend(&mut self, other: Self) {
        self.output.results.extend(other.output.results);
    }

    fn print(&self) {
        println!(" Report of OpenBabel Substructure Search\n");
        println!(" Input");
        println!("\t smarts: {}", self.input.smarts);
        println!(" Operator");
        println!(" Dataset");
        println!("\t kind: {}", self.dsk);
        println!(" Output");
        for (matches, id) in self.output.results.iter() {
            println!("\t {id}\t {matches:?}");
        }
        println!("\t Count: {}", self.output.results.len());
    }
}

