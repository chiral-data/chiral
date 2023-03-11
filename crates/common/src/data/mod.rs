mod doc;
mod source;

pub use doc::smiles::DocSMILES;
pub use doc::smiles::DocStoreSMILES;
pub use source::chembl::SourceChembl;

pub mod types {
    pub type EntryID = String;
}

pub trait Dummy {
    fn dummy() -> Self;
} 

pub trait Info {
    fn info(&self) -> String;
}

pub fn load_from_path<T: Dummy + From<SourceChembl>>(kind: &crate::kinds::Dataset, filepath: &std::path::PathBuf) -> T {
    match kind {
        crate::kinds::Dataset::Empty => panic!(),
        crate::kinds::Dataset::Dummy => T::dummy(),
        crate::kinds::Dataset::TestChembl | crate::kinds::Dataset::Chembl30 => {
            let mut sc = SourceChembl::new();
            sc.set_path(filepath.as_os_str());
            sc.load_all();
            T::from(sc)
        },
        crate::kinds::Dataset::PubChem => {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let filepath = std::path::PathBuf::from("../../../chiral-db-example-data/ChEMBL/chembl_30_chemreps_100.txt");
        let doc_1 = load_from_path::<doc::smiles::DocSMILES>(&crate::kinds::Dataset::TestChembl, &filepath);
        assert_eq!(doc_1.get_ids().len(), 100);
        assert_eq!(doc_1.get_smiles_vec().len(), 100);
    }
}

