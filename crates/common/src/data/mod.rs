mod doc;
mod source;

pub use doc::smiles::DocSMILES;
pub use doc::smiles::DocStoreSMILES;
pub use source::chembl::SourceChembl;

pub mod types {
    pub type EntryID = String;
}

pub trait Empty {
    fn empty() -> Self;
}

pub trait Dummy {
    fn dummy() -> Self;
} 

pub trait Info {
    fn info(&self) -> String;
}

pub fn load_from_path<T: Empty + Dummy + From<SourceChembl>>(kind: &crate::kinds::Dataset, data_dir: &std::path::PathBuf) -> T {
    match kind {
        crate::kinds::Dataset::Empty => T::empty(), 
        crate::kinds::Dataset::Dummy => T::dummy(),
        crate::kinds::Dataset::TestChembl | crate::kinds::Dataset::Chembl30 => {
            let mut sc = SourceChembl::new();
            sc.set_path(data_dir.join(kind.filename()).as_os_str());
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
        let data_dir = std::path::PathBuf::from("../../../chiral-db-example-data/ChEMBL");
        let doc_1 = load_from_path::<doc::smiles::DocSMILES>(&crate::kinds::Dataset::TestChembl, &data_dir);
        assert_eq!(doc_1.get_ids().len(), 10000);
        assert_eq!(doc_1.get_smiles_vec().len(), 10000);
    }
}

