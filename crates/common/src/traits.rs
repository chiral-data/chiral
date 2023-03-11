//! Traits of chiral: Serialization, Operator
//! 

pub type SerializedFormat = String;

pub trait Serialization {
    fn ser_to(&self) -> SerializedFormat; 
    fn ser_from(content: &SerializedFormat) -> Self; 
}

pub trait TraitOperator {
    type InputType;
    type DataType;
    type OutputType;
    type ReportType;

    fn compute(&self, input: &Self::InputType, data: &Self::DataType) -> Self::OutputType;
    fn report(&self, input: Self::InputType, data: &Self::DataType, output: Self::OutputType) -> Self::ReportType;
}

pub trait TraitData {
    fn blank() -> Self;
    fn len(&self) -> usize;
}

pub trait TraitOutput: Serialization {
    fn blank() -> Self;
    fn clear(&mut self);
    fn append(&mut self, other: &mut Self);
    fn len(&self) -> usize;
}

pub trait TraitReport: Serialization {
    fn extend(&mut self, other: Self);
    fn print(&self);
    fn save(&self, filepath: &std::path::PathBuf) -> Result<u64, std::io::Error> {
        std::fs::File::create(filepath)
            .and_then(|mut dest| {
                std::io::copy(&mut self.ser_to().as_bytes(), &mut dest)
            })
    }
}

pub trait TraitComputingUnit {
    fn set_input(&mut self, input_ser: &crate::types::ser::InputSer);
    fn set_data(&mut self, dsk: &crate::kinds::Dataset, doc: &crate::data::DocSMILES);
    fn compute(&mut self);
    fn get_output_len(&self) -> usize;
}