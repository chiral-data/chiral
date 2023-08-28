//! ReCGen Buillding
//! 

use serde::{Serialize, Deserialize};
use chiral_derive::*;
use crate::traits::*;

/// Input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization, InputFileRequirements)] 
pub struct Input {
    pub db_file: String,
    pub mol: String, // moleclue in mol format
}

impl TraitInput for Input {
    fn default() -> Self {
        let data_dir = crate::apps::env_var::Variable::ChiralDataDir.get();
        let input_dir =  std::path::PathBuf::from(data_dir).join("recgen/inputs");
        let mol = std::fs::read_to_string(input_dir.join("sample4.mol")).unwrap();
        Self { db_file: "DrugBank_M.db".to_string(), mol } 
    }
}

/// Output
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, Clone)]
pub struct Output {
    pub smiles: Vec<crate::apps::types::SMILES>
}

impl TraitOutput for Output {
    fn blank() -> Self { Self { smiles: vec![] } }

    fn len(&self) -> usize { self.smiles.len() }

    fn clear(&mut self) {
        self.smiles.clear();
    }

    fn append(&mut self, other: &mut Self) {
        for r in other.smiles.to_vec().into_iter() {
            if !self.smiles.contains(&r) {
                self.smiles.push(r);
            }
        }
    }
}


/// Report
#[add_report_fields]
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, ReportRequirements)]
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
        for smiles in self.output.smiles.iter() {
            println!("\t {smiles}");
        }
        println!("\t Count: {}", self.output.len());
    }
}
