//! Document SMILES
//!     standardized database with id and SMILES
//!     converted from other datasets

use serde::{Serialize, Deserialize}; 
use crate::traits::{Serialization, SerializedFormat};
use chiral_derive::Serialization;

#[derive(Serialize, Deserialize, Serialization)]
pub struct DocSMILES {
    ids: Vec<crate::data::types::EntryID>,
    smiles: Vec<crate::app::chem::types::SMILES>,
}

impl DocSMILES {
    pub fn empty() -> Self {
        Self { ids: vec![], smiles: vec![] }
    }

    pub fn new(ids_in: Vec<crate::data::types::EntryID>, smiles_in: Vec<crate::app::chem::types::SMILES>) -> Self {
        let mut perm = permutation::sort(&ids_in);
        let mut ids = ids_in;
        perm.apply_slice_in_place(&mut ids);
        let mut smiles = smiles_in;
        perm.apply_slice_in_place(&mut smiles);

        Self { ids, smiles }
    }

    pub fn get_smiles(&self, id: &crate::data::types::EntryID) -> Option<&crate::app::chem::types::SMILES> {
        match self.ids.binary_search(id) {
            Ok(index) => self.smiles.get(index),
            Err(_) => None
        }
    }

    pub fn extract_ids(&self, range: &std::ops::Range<usize>) -> Vec<crate::data::types::EntryID> { self.get_ids().as_slice()[range.to_owned()].to_vec() }
    pub fn extract_smiles_vec(&self, range: &std::ops::Range<usize>) -> Vec<crate::app::chem::types::SMILES> { self.get_smiles_vec().as_slice()[range.to_owned()].to_vec() }
    pub fn extract(&self, range: &std::ops::Range<usize>) -> Self { Self::new(self.extract_ids(range), self.extract_smiles_vec(range)) }

    pub fn get_smiles_vec(&self) -> &Vec<crate::app::chem::types::SMILES> { &self.smiles }
    pub fn get_ids(&self) -> &Vec<crate::data::types::EntryID> { &self.ids }
    pub fn len(&self) -> usize { self.ids.len() }
}

impl crate::data::Empty for DocSMILES {
    fn empty() -> Self {
        Self::new(vec![], vec![])
    }
}

impl crate::data::Dummy for DocSMILES {
    fn dummy() -> Self {
        let ids = vec![
            "label_1".to_string(),
            "label_3".to_string(),
            "label_2".to_string(),
            "label_4".to_string()
        ];
        let smiles = vec![
            String::from("O=C(C)Oc1ccccc1C(=O)O"),
            String::from("N1=C(c3c(Sc2c1cccc2)cccc3)N4CCN(CCOCCO)CC4"),
            String::from("O=C(O)C[C@H](O)C[C@H](O)CCn2c(c(c(c2c1ccc(F)cc1)c3ccccc3)C(=O)Nc4ccccc4)C(C)C"),
            String::from("CC(=O)Nc1ccc(O)cc1")
        ];

        Self::new(ids, smiles)
    }
}

impl From<crate::data::SourceChembl> for DocSMILES {
    fn from(sc: crate::data::SourceChembl) -> Self {
        let (smiles, ids) = sc.get_smiles_id_pairs();
        DocSMILES::new(ids, smiles)
    }
}

/// Datastore for DocSMILES
pub type DocStoreSMILES = std::collections::HashMap<crate::kinds::Dataset, DocSMILES>;

impl crate::data::Info for DocStoreSMILES {
    fn info(&self) -> String {
        format!("{:15} {:>15}\n", "name", "entries")
        + vec!["=";31].join("").as_str() + "\n"
        + self.iter()
            .map(|(k, v)| format!("{:15} {:15}", k, v.len()))
            .collect::<Vec<String>>()
            .join("\n")
            .as_str()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Dummy, Info};

    #[test]
    fn test_doc_smiles() {
        let doc = DocSMILES::dummy();
        assert_eq!(doc.ids.len(), 4);
        assert_eq!(doc.smiles.len(), 4);
        assert_eq!(doc.get_smiles(&"label_3".to_string()), Some(&String::from("N1=C(c3c(Sc2c1cccc2)cccc3)N4CCN(CCOCCO)CC4")));
        assert_eq!(doc.get_smiles(&"label_5".to_string()), None); 
    }

    #[test]
    fn test_chembl() {
        let mut sc = crate::data::SourceChembl::new();
        sc.set_path(&std::ffi::OsString::from("../../../chiral-db-example-data/ChEMBL/chembl_30_chemreps_100.txt"));
        sc.load_all();
        let doc = DocSMILES::from(sc);
        assert_eq!(doc.ids.len(), 100);
        assert_eq!(doc.smiles.len(), 100);
    }

    #[test]
    fn test_store() {
        let mut store = DocStoreSMILES::new();
        let doc_1 = DocSMILES::dummy();
        store.insert(crate::kinds::Dataset::Dummy, doc_1);
        let mut sc = crate::data::SourceChembl::new();
        sc.set_path(&std::ffi::OsString::from("../../../chiral-db-example-data/ChEMBL/chembl_30_chemreps_10k.txt"));
        sc.load_all();
        let doc_2 = DocSMILES::from(sc);
        store.insert(crate::kinds::Dataset::TestChembl, doc_2);
        println!("{}", store.info());
        assert!(store.info().contains("test_chembl"));
        assert!(store.info().contains("dummy"));
        assert!(store.info().contains("4"));
        assert!(store.info().contains("100"));
    }

    #[test]
    fn test_permutation() {
        let dsk = crate::kinds::Dataset::TestChembl;
        let filepath = std::path::PathBuf::from("../../../chiral-db-example-data/ChEMBL");
        let doc = crate::data::load_from_path::<DocSMILES>(&dsk, &filepath);
        let id = "CHEMBL10030".to_string();
        assert_eq!(doc.get_smiles(&id).unwrap().to_string(), "O=C(c1ccc(OCCN2CCCC2)cc1)c1c(-c2ccc(O)cc2)sc2cc(O)ccc12".to_string());
    }
}