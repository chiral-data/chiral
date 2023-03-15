//!  OpenBabel Substructure Match
//! 

use chiral_common::app::chem::openbabel::substructure::*;
use chiral_common::traits::*;

/// Data
pub struct Data {
    dsk: chiral_common::kinds::Dataset,
    ids: Vec<chiral_common::data::types::EntryID>,
    mols: Vec<openbabel::molecule::Molecule>
}

impl Data {
    fn new(dsk: chiral_common::kinds::Dataset, (ids, smiles_vec): chiral_common::app::chem::types::IdSmilesPairs) -> Self {
        let mols = smiles_vec.iter()
            .map(|smiles| openbabel::molecule::Molecule::new_from_smiles(smiles))
            .collect();
        Self { dsk, ids, mols }
    }
}

impl TraitData for Data {
    fn blank() -> Self {
        Self { dsk: chiral_common::kinds::Dataset::Empty, ids: vec![], mols: vec![] }
    }

    fn len(&self) -> usize { self.ids.len() }
}

/// Operator 
struct OpenBabelSSMatcher {
    sp: openbabel::smartspattern::SmartsPattern
}

impl OpenBabelSSMatcher {
    pub fn new(smarts: &chiral_common::app::chem::types::SMARTS) -> Self {
        let sp = openbabel::smartspattern::SmartsPattern::new_from_smarts(smarts);
        Self { sp }
    }

    pub fn find_match(&self, mol: &openbabel::molecule::Molecule) -> MatchResult {
        self.sp.find_match(mol)
    }
}

pub struct Operator {
}

impl TraitOperator for Operator {
    type InputType = Input; 
    type DataType = Data;
    type OutputType = Output; 
    type ReportType = Report;

    fn new(opk: &chiral_common::kinds::Operator) -> Self {
        match opk {
            chiral_common::kinds::Operator::OpenBabelSSMatching => Self {},
            _ => panic!("Operator kind mismatch")
        }
    }

    fn get_kind(&self) -> chiral_common::kinds::Operator {
        chiral_common::kinds::Operator::OpenBabelSSMatching
    }

    fn prepare_data(&self, dsk: &chiral_common::kinds::Dataset, div_index: &chiral_common::job::DividendIndex, ds: std::sync::Arc<std::sync::Mutex<dyn TraitDataStore>>) -> Option<Self::DataType> {
        ds.lock().unwrap()
            .get_id_smiles_pairs(dsk, div_index)
            .map(|(ids, smiles_vec)| Self::DataType::new(dsk.to_owned(), (ids, smiles_vec)))
    }

    fn compute(&self, input: &Self::InputType, data: &Self::DataType, _div_index: &chiral_common::job::DividendIndex) -> Self::OutputType {
        let matcher = OpenBabelSSMatcher::new(&input.smarts);
        let results = data.mols.iter()
            .map(|mol| matcher.find_match(mol))
            .zip(data.ids.iter())
            .filter(|(mr, _)| mr.len() > 0)
            .map(|(mr, id)| (mr, id.to_string()))
            .collect();

        Output { results }
    }

    fn report(&self, job_id: chiral_common::job::ID, input: Self::InputType, data: &Self::DataType, output: Self::OutputType) -> Self::ReportType {
        Report {
            job_id,
            cuk: chiral_common::kinds::ComputingUnit::new(self.get_kind(), data.dsk),
            input,
            output
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chiral_common::data::Dummy;

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
        let dsk = chiral_common::kinds::Dataset::Dummy;
        let doc = chiral_common::data::DocSMILES::dummy();
        let data = Data::new(dsk.to_owned(), (doc.get_ids().to_vec(), doc.get_smiles_vec().to_vec()));
        assert_eq!(data.len(), 4);
        let opk = chiral_common::kinds::Operator::OpenBabelSSMatching;
        let op = Operator::new(&opk);
        let input = Input { smarts: String::from("C(=O)O") };
        let output = op.compute(&input, &data, &(0, 1));
        assert_eq!(output.len(), 2); 
        let report = op.report("job_id".to_string(), input, &data, output);
        let serialized_report = report.ser_to();
        let report_deserialized = Report::ser_from(&serialized_report);
        let cuk = chiral_common::kinds::ComputingUnit::new(chiral_common::kinds::Operator::OpenBabelSSMatching, dsk);
        assert_eq!(report_deserialized.cuk, cuk);
        assert_eq!(report_deserialized.output.len(), 2);
        let input_1 = Input { smarts: String::from("C(=O)O") };
        let output_1 = op.compute(&input_1, &data, &(0, 1));
        let report_1 = op.report("job_id".to_string(), input_1, &data, output_1);
        assert_eq!(report_1.output.len(), 2); 
        let input_2 = Input { smarts: String::from("C(=O)O") };
        let output_2 = op.compute(&input_2, &data, &(0, 1));
        let report_2 = op.report("job_id".to_string(), input_2, &data, output_2);
        assert_eq!(report_2.output.len(), 2); 
    }
}