//! OpenBabel Similarity Search
//! 

use chiral_common::app::chem::openbabel::similarity::*;
use chiral_common::traits::*;

/// Tanimoto coefficient for Fingerprint
fn similarity_tanimoto(fpd_1: &[u32], fpd_2: &[u32]) -> f32 {
    let mut andbits: u32 = 0;
    let mut orbits: u32 = 0;
    for i in 0..fpd_1.len() {
        let andfp: u32 = fpd_1[i] & fpd_2[i];
        let orfp: u32 = fpd_1[i] | fpd_2[i];
        andbits += andfp.count_ones();
        orbits += orfp.count_ones();
    }

    if orbits > 0 {
        andbits as f32 / orbits as f32
    } else {
        0.0
    }
}

/// Data
pub struct Data {
    dsk: chiral_common::kinds::Dataset,
    ids: Vec<chiral_common::data::types::EntryID>,
    fps: Vec<chiral_common::app::chem::types::FingerprintData>
}

impl Data {
    fn new(dsk: chiral_common::kinds::Dataset, (ids, smiles_vec): chiral_common::app::chem::types::IdSmilesPairs, fpk: &chiral_common::app::chem::kinds::Fingerprint) -> Self {
        let fpk_ob = super::to_ob_fp_kind(fpk);
        let fpg  = openbabel::fingerprint::FingerprintGenerator::new(fpk_ob);
        let fps = fpg.get_fingerprint_for_smiles_vec(&smiles_vec);
        Self { dsk, ids, fps }
    }
}

impl TraitData for Data {
    fn blank() -> Self {
        Self { dsk: chiral_common::kinds::Dataset::Empty, ids: vec![], fps: vec![] }
    }

    fn len(&self) -> usize {
        self.ids.len()
    }
}

/// Operator
pub struct Operator {
    fpk: chiral_common::app::chem::kinds::Fingerprint,
    fpg: openbabel::fingerprint::FingerprintGenerator,
}

impl chiral_common::traits::TraitOperator for Operator {
    type InputType = Input;
    type DataType = Data;
    type OutputType = Output; 
    type ReportType = Report;

    fn new(opk: &chiral_common::kinds::Operator) -> Self {
        match opk {
            chiral_common::kinds::Operator::OpenBabelSimilaritySearching(fpk) => {
                let fpk_ob = super::to_ob_fp_kind(&fpk);
                let fpg = openbabel::fingerprint::FingerprintGenerator::new(fpk_ob);
                Self { fpk: fpk.to_owned(), fpg }
            },
            _ => panic!("Operator Kind mismatch")
        }
    }

    fn get_kind(&self) -> chiral_common::kinds::Operator {
        chiral_common::kinds::Operator::OpenBabelSimilaritySearching(self.fpk.to_owned())
    }

    fn prepare_data(&self, dsk: &chiral_common::kinds::Dataset, div_index: &chiral_common::job::DividendIndex, ds: std::sync::Arc<std::sync::Mutex<dyn TraitDataStore>>) -> Option<Self::DataType> {
        ds.lock().unwrap()
            .get_id_smiles_pairs(dsk, div_index)
            .map(|id_smiles_pairs| Self::DataType::new(dsk.to_owned(), id_smiles_pairs, &self.fpk))
    }

    fn compute(&self, input: &Self::InputType, data: &Self::DataType, _div_index: &chiral_common::job::DividendIndex) -> Self::OutputType {
        let mol = openbabel::molecule::Molecule::new_from_smiles(&input.smiles);
        let fp_target = self.fpg.get_fingerprint(&mol);
        let results = data.fps.iter()
            .map(|fp| similarity_tanimoto(fp, &fp_target))
            .zip(data.ids.iter())
            .filter(|(coeff, _)| *coeff > input.threshold)
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
    fn test_op() {
        let dsk = chiral_common::kinds::Dataset::Dummy;
        let doc_smiles = chiral_common::data::DocSMILES::dummy();
        let com_fpk = chiral_common::app::chem::kinds::Fingerprint::kind_openbabel_ecfp4(2048);
        let opk = chiral_common::kinds::Operator::OpenBabelSimilaritySearching(com_fpk.to_owned());
        let op = Operator::new(&opk);
        let data = Data::new(dsk.to_owned(), (doc_smiles.get_ids().to_vec(), doc_smiles.get_smiles_vec().to_owned()), &com_fpk);
        assert_eq!(data.len(), 4);
        let input = Input { smiles: String::from("c1ccccc1"), threshold: 0.045 };
        let output = op.compute(&input, &data, &(0, 1));
        assert_eq!(output.len(), 2); 
        let report = op.report("job_id".to_string(), input, &data, output);
        let serialized_report = report.ser_to();
        let report_deserialized = Report::ser_from(&serialized_report);
        let cuk = chiral_common::kinds::ComputingUnit::new(chiral_common::kinds::Operator::OpenBabelSimilaritySearching(com_fpk), dsk);
        assert_eq!(report_deserialized.cuk, cuk);
        assert_eq!(report_deserialized.output.len(), 2);
    }
}
