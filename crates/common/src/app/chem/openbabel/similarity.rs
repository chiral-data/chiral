//! OpenBabel Similarity Search

use serde::{Serialize, Deserialize};
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive_new::Serialization;

/// Input
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization)] 
pub struct Input {
    pub smiles: crate::app::chem::types::SMILES,
    pub threshold: f32
}

impl std::default::Default for Input {
    fn default() -> Self {
        Self { 
            smiles: crate::app::chem::types::SMILES::from("c1ccccc1N=O"),
            threshold: 0.1
        }
    }
}


/// Output
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, Clone)]
pub struct Output {
    pub results: Vec<(f32, crate::data::types::EntryID)>
}

impl crate::traits::TraitOutput for Output {
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization)]
pub struct Report {
    pub input: Input,
    pub fpk: crate::app::chem::kinds::Fingerprint,
    pub dsk: crate::kinds::Dataset,
    pub output: Output
}

impl crate::traits::TraitReport for Report {
    fn extend(&mut self, other: Self) {
        self.output.results.extend(other.output.results);
    }

    fn print(&self) {
        println!("Report of OpenBabel Similarity Search\n");
        println!(" Input");
        println!("\t smiles: {}", self.input.smiles);
        println!("\t threshold: {:.2}", self.input.threshold);
        println!(" Operator");
        println!("\t fingerprint kind: {}", self.fpk.to_string());
        println!(" Dataset");
        println!("\t kind: {}", self.dsk);
        println!(" Output");
        for (coeff, id) in self.output.results.iter() {
            println!("\t {id}\t {coeff:.3}");
        }
        println!("\t Count: {}", self.output.results.len());
    }
}