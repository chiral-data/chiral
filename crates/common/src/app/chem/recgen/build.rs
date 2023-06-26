//! ReCGen Buillding
//! 

use serde::{Serialize, Deserialize};
use chiral_derive::*;
use crate::traits::*;

/// Input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization, InputFileRequirements)] 
pub struct Input {
    pub mol: String, // moleclue in mol format
}

impl TraitInput for Input {
    fn default() -> Self {
        // todo
        // the requirement of input directory could has to be removed
        let input_dir_o = std::env::var_os("RECGEN_INPUT");
        if input_dir_o.is_none() {
            panic!("Set environment variable RECGEN_INPUT for example input directory");
        }
        
        let input_dir =  std::path::PathBuf::from(input_dir_o.unwrap().into_string().unwrap());
        let mol = std::fs::read_to_string(input_dir.join("sample4.mol")).unwrap();
        Self { mol } 
    }
}

/// Output
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, Clone)]
pub struct Output {
    pub results: Vec<crate::app::chem::types::SMILES>
}

impl TraitOutput for Output {
    fn blank() -> Self { Self { results: vec![] } }

    fn len(&self) -> usize { self.results.len() }

    fn clear(&mut self) {
        self.results.clear();
    }

    fn append(&mut self, other: &mut Self) {
        for r in other.results.to_vec().into_iter() {
            if !self.results.contains(&r) {
                self.results.push(r);
            }
        }
    }
}


/// Report
#[add_report_fields]
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, ImplReport)]
pub struct Report {}

impl TraitReport for Report {
    fn print(&self) {
        println!(" Report of ReCGen Build\n");
        println!(" Input");
        println!("\t input mol: {}", self.input.mol);
        println!(" Operator");
        println!(" Dataset");
        println!("\t kind: {}", self.cuk.get_dsk());
        println!(" Output");
        for smiles in self.output.results.iter() {
            println!("\t {smiles}");
        }
        println!("\t Count: {}", self.output.len());
    }
}
