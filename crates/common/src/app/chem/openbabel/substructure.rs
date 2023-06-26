//! OpenBabel Substructure Match
//! 

use serde::{Serialize, Deserialize};
use crate::traits::*;
use chiral_derive::*;

/// Input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization, InputFileRequirements)] 
pub struct  Input {
    pub smarts: crate::app::chem::types::SMARTS
}

impl TraitInput for Input {
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
#[add_report_fields]
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, ImplReport)]
pub struct Report {}

impl crate::traits::TraitReport for Report {
    fn print(&self) {
        println!(" Report of OpenBabel Substructure Search\n");
        println!(" Input");
        println!("\t smarts: {}", self.input.smarts);
        println!(" Operator");
        println!(" Dataset");
        println!("\t kind: {}", self.cuk.get_dsk());
        println!(" Output");
        for (matches, id) in self.output.results.iter() {
            println!("\t {id}\t {matches:?}");
        }
        println!("\t Count: {}", self.output.results.len());
        // let mut ids: Vec<crate::data::types::EntryID> = vec![];
        // for r in self.output.results.iter() {
        //     if !ids.contains(&r.1) {
        //         ids.push(r.1.to_owned());
        //     }
        // }
        // println!("\t Count: {} after duplication", ids.len()); 
    }
}

