//! Dataset Kind

use serde::{Serialize, Deserialize};
use chiral_derive::Serialization;
use crate::traits::{Serialization, SerializedFormat};
use strum_macros::{EnumString, Display};

#[derive(Serialize, Deserialize, Serialization, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]  
#[strum(serialize_all = "snake_case")]
pub enum Kind {
    #[strum(serialize = "empty")]
    Empty,
    #[strum(serialize = "dummy")]
    Dummy,
    #[strum(serialize = "test_chembl")]
    TestChembl,
    #[strum(serialize = "chembl30")]
    Chembl30,
    #[strum(serialize = "pub_chem")]
    PubChem
}

impl Kind {
    // pub fn serialize(&self) -> crate::SerializedFormat { serde_json::to_string(self).unwrap() }
    // pub fn deserialize(content: &crate::SerializedFormat) -> Self { serde_json::from_str(content).unwrap() }

    pub fn size(&self) -> usize {
        match self {
            Kind::Empty => 0,
            Kind::Dummy => 4,
            Kind::TestChembl => 10000,
            Kind::Chembl30 => 2136187,
            Kind::PubChem => 160000000, // rough figure, PubChem does not have a version 
        }
    }

    pub fn env_key(&self) -> &str {
        match self {
            Kind::Empty => unimplemented!(),
            Kind::Dummy => unimplemented!(),
            Kind::TestChembl | Kind::Chembl30 => "DATASET_CHEMBL_PATH",
            Kind::PubChem => ""
        }
    }

    pub fn source_url(&self) -> &str {
        match self {
            Kind::Empty => unimplemented!(),
            Kind::Dummy => unimplemented!(),
            Kind::TestChembl => "https://github.com/chiral-data/chiral-db-example-data/blob/main/ChEMBL/chembl_30_chemreps_10k.txt?raw=true",
            Kind::Chembl30 => "https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/releases/chembl_30/chembl_30_chemreps.txt.gz",
            Kind::PubChem => "https://ftp.ncbi.nlm.nih.gov/pubchem/Compound/Extras/CID-SMILES.gz"
        }
    }

    pub fn filename(&self) -> &str {
        match self {
            Kind::Empty => unimplemented!(),
            Kind::Dummy => unimplemented!(),
            Kind::TestChembl => "chembl_30_chemreps_10k.txt",
            Kind::Chembl30 => "chembl_30_chemreps.txt",
            Kind::PubChem => "CID-SMILES.gz"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;
    use std::str::FromStr;

    #[test]
    fn test_kind() {
        let kind = Kind::TestChembl;
        assert_eq!(kind.size(), 10000);
    }

    #[test]
    fn test_strum() {
        assert_eq!(Kind::from_str("empty").unwrap(), Kind::Empty);
        assert_eq!(Kind::from_str("test_chembl").unwrap(), Kind::TestChembl);
        assert_eq!(Kind::from_str("chembl30").unwrap(), Kind::Chembl30);
        assert_eq!(Kind::Empty.to_string(), "empty");
        assert_eq!(Kind::TestChembl.to_string(), "test_chembl");
        assert_eq!(Kind::Chembl30.to_string(), "chembl30");
    }

}

