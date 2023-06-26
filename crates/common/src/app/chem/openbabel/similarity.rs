//! OpenBabel Similarity Search

use serde::{Serialize, Deserialize};
use crate::traits::*;
use chiral_derive::*;

/// Input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization, InputFileRequirements)] 
pub struct Input {
    pub smiles: crate::app::chem::types::SMILES,
    pub threshold: f32,
}

impl TraitInput for Input {
    fn default() -> Self {
        Self { 
            smiles: crate::app::chem::types::SMILES::from("c1ccccc1N=O"),
            threshold: 0.1
        }
    }
}

/// Output
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization)]
pub struct Output {
    pub results: Vec<(f32, crate::data::types::EntryID)>
}

impl TraitOutput for Output {
    fn blank() -> Self { Self { results: vec![] } }

    fn len(&self) -> usize { self.results.len() }

    fn clear(&mut self) {
        self.results.clear();
    }

    fn append(&mut self, other: &mut Self) {
        self.results.append(&mut other.results)
    }
}

/// Report
#[add_report_fields]
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, ImplReport)]
pub struct Report {}

impl crate::traits::TraitReport for Report {
    fn print(&self) {
        println!("Report of OpenBabel Similarity Search\n");
        println!(" Input");
        println!("\t smiles: {}", self.input.smiles);
        println!("\t threshold: {:.2}", self.input.threshold);
        println!(" Operator");
        println!("\t fingerprint kind: {}", self.cuk.get_opk().to_string());
        println!(" Dataset");
        println!("\t kind: {}", self.cuk.get_dsk());
        println!(" Output");
        for (coeff, id) in self.output.results.iter() {
            println!("\t {id}\t {coeff:.3}");
        }
        println!("\t Count: {}", self.output.len());
    }
}