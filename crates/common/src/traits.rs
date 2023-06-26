//! Traits of chiral: Serialization, Operator
//! 

pub type SerializedFormat = String;

pub trait Serialization {
    fn ser_to(&self) -> SerializedFormat; 
    fn ser_from(content: &SerializedFormat) -> Self; 
}

pub trait TraitFileRequirements {
    fn dir(&self) -> String { ".".to_string() }
    fn dir_full(&self) -> String { ".".to_string() }
    fn files_in(&self) -> Vec<String> { vec![] }
    fn files_out(&self) -> Vec<String> { vec![] }
}

pub trait TraitData {
    fn blank() -> Self;
    fn len(&self) -> usize;
}

pub trait TraitDataStore {
    fn get_id_smiles_pairs(&self, dsk: &crate::kinds::Dataset, div_index: &crate::job::DividendIndex) -> Option<crate::app::chem::types::IdSmilesPairs>;
}

pub trait TraitFileClient : std::marker::Sync + std::marker::Send {
    fn download_files(&self, _local_dir: &str, _remote_dir: &str, _files: &Vec<String>) -> anyhow::Result<()> { Ok(()) }
    fn upload_files(&self, _local_dir: &str, _remote_dir: &str, _files: &Vec<String>) -> anyhow::Result<()> { Ok(()) }
    fn remove_local_files(&self, _local_dir: &str, _files: &Vec<String>) -> anyhow::Result<()> { Ok(()) }
    fn remove_local_dir(&self, _local_dir: &str) -> anyhow::Result<()> { Ok(()) }
}

pub trait TraitOperator {
    type InputType;
    type DataType;
    type OutputType;
    type ReportType;

    fn new(opk: &crate::kinds::Operator) -> Self;
    fn get_kind(&self) -> crate::kinds::Operator;
    fn prepare_data(&self, dsk: &crate::kinds::Dataset, div_index: &crate::job::DividendIndex, ds: std::sync::Arc<std::sync::Mutex<dyn TraitDataStore>>) -> Option<Self::DataType>;
    fn compute(&self, input: &Self::InputType, data: &Self::DataType, div_index: &crate::job::DividendIndex) -> Self::OutputType;
    fn report(&self, job_id: crate::job::ID, input: Self::InputType, data: &Self::DataType, output: Self::OutputType) -> Self::ReportType;
}

pub trait TraitInput: Serialization {
    fn default() -> Self;
}

pub trait TraitOutput: Serialization {
    fn blank() -> Self;
    fn clear(&mut self);
    fn append(&mut self, other: &mut Self);
    fn len(&self) -> usize;
}

pub trait TraitReport: Serialization {
    fn print(&self);
    fn save(&self, filepath: &std::path::PathBuf) -> std::io::Result<u64> {
        std::fs::File::create(filepath)
            .and_then(|mut dest| {
                std::io::copy(&mut self.ser_to().as_bytes(), &mut dest)
            })
    }
}