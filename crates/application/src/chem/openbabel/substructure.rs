//!  OpenBabel Substructure Match
//! 

use chiral_common_new::app::chem::openbabel::substructure::{Input, Output, MatchResult, Report};
use chiral_common_new::traits::*;

/// Data
pub struct Data {
    dsk: chiral_common_new::kinds::Dataset,
    ids: Vec<chiral_common_new::data::types::EntryID>,
    mols: Vec<openbabel::molecule::Molecule>
}

impl From<(&chiral_common_new::kinds::Dataset, &chiral_common_new::data::DocSMILES)> for Data {
    fn from((dsk_in, doc): (&chiral_common_new::kinds::Dataset, &chiral_common_new::data::DocSMILES)) -> Self {
        let mols = doc.get_smiles_vec().iter()
            .map(|smiles| openbabel::molecule::Molecule::new_from_smiles(smiles))
            .collect();
        let ids = doc.get_ids().to_vec();
        Self { dsk: dsk_in.to_owned(), ids, mols }
    }
}

impl chiral_common_new::traits::TraitData for Data {
    fn blank() -> Self {
        Self { dsk: chiral_common_new::kinds::Dataset::Empty, ids: vec![], mols: vec![] }
    }

    fn len(&self) -> usize {
        self.ids.len()
    }
}

/// Operator 
struct OpenBabelSSMatcher {
    sp: openbabel::smartspattern::SmartsPattern
}

impl OpenBabelSSMatcher {
    pub fn new(smarts: &chiral_common_new::app::chem::types::SMARTS) -> Self {
        let sp = openbabel::smartspattern::SmartsPattern::new_from_smarts(smarts);
        Self { sp }
    }

    pub fn find_match(&self, mol: &openbabel::molecule::Molecule) -> MatchResult {
        self.sp.find_match(mol)
    }
}

pub struct Operator {
}

impl Operator {
    pub fn new() -> Self {
        Self {}
    }
}

impl chiral_common_new::traits::TraitOperator for Operator {
    type InputType = Input; 
    type DataType = Data;
    type OutputType = Output; 
    type ReportType = Report;

    fn compute(&self, input: &Self::InputType, data: &Self::DataType) -> Self::OutputType {
        let matcher = OpenBabelSSMatcher::new(&input.smarts);
        let results = data.mols.iter()
            .map(|mol| matcher.find_match(mol))
            .zip(data.ids.iter())
            .filter(|(mr, _)| mr.len() > 0)
            .map(|(mr, id)| (mr, id.to_string()))
            .collect();

        Output { results }
    }

    fn report(&self, input: Self::InputType, data: &Self::DataType, output: Self::OutputType) -> Self::ReportType {
        Report {
            input,
            dsk: data.dsk.to_owned(),
            output
        }
        
    }
}

/// Computing Unit
pub struct ComputingUnit {
    op: Operator,
    data: Data,
    input: Input,
    output: Output
}

impl ComputingUnit {
    pub fn new() -> Self {
        let op = Operator::new();
        Self { op, input: Input::default(), data: Data::blank(), output: Output::blank() }
    }
}

impl chiral_common_new::traits::TraitComputingUnit for ComputingUnit {
    fn set_input(&mut self, input_ser: &chiral_common_new::types::ser::InputSer) {
        self.input = Input::ser_from(input_ser);
    }

    fn set_data(&mut self, dsk: &chiral_common_new::kinds::Dataset, doc: &chiral_common_new::data::DocSMILES) {
        self.data = Data::from((dsk, doc));
    }

    fn compute(&mut self) {
        let mut output = self.op.compute(&self.input, &self.data);
        self.output.append(&mut output);
    }

    fn get_output_len(&self) -> usize {
        self.output.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chiral_common_new::data::Dummy;

    #[test]
    fn test_matcher() {
        let matcher = OpenBabelSSMatcher::new(&String::from("O=CN*"));
        let mol_1 = openbabel::molecule::Molecule::new_from_smiles("NCC(=O)NCC");
        let match_result_1 = matcher.find_match(&mol_1);
        assert_eq!(match_result_1.len(), 1);
        assert_eq!(vec![vec![4, 3, 5, 6]], match_result_1);
        let matcher_2 = OpenBabelSSMatcher::new(&String::from("c1ccccc1N=O"));
        let mol_2 = openbabel::molecule::Molecule::new_from_smiles("COc1cc([N+](=O)[O-])c(OC)cc1CC(C)N");
        let match_result_2 = matcher_2.find_match(&mol_2);
        assert_eq!(match_result_2.len(), 2);
        assert_eq!(vec![vec![4, 3, 13, 12, 9, 5, 6, 7], vec![9, 12, 13, 3, 4, 5, 6, 7]], match_result_2);
    }

    #[test]
    fn test_op() {
        let dsk = chiral_common_new::kinds::Dataset::Dummy;
        let doc_smiles = chiral_common_new::data::DocSMILES::dummy();
        let data = Data::from((&dsk, &doc_smiles));
        assert_eq!(data.len(), 4);
        let op = Operator::new();
        let input = Input { smarts: String::from("C(=O)O") };
        let output = op.compute(&input, &data);
        assert_eq!(output.len(), 2); 
        let report = op.report(input, &data, output);
        let serialized_report = report.ser_to();
        let report_deserialized = Report::ser_from(&serialized_report);
        assert_eq!(report_deserialized.dsk, dsk);
        assert_eq!(report_deserialized.output.len(), 2);
        let input_1 = Input { smarts: String::from("C(=O)O") };
        let output_1 = op.compute(&input_1, &data);
        let mut report_1 = op.report(input_1, &data, output_1);
        let input_2 = Input { smarts: String::from("C(=O)O") };
        let output_2 = op.compute(&input_2, &data);
        let report_2 = op.report(input_2, &data, output_2);
        report_1.extend(report_2);
        assert_eq!(report_1.output.len(), 4); 
    }
}