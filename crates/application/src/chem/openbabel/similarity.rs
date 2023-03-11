use chiral_common_new::app::chem::openbabel::similarity::{Input, Output, Report};
use chiral_common_new::traits::*;

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
    dsk: chiral_common_new::kinds::Dataset,
    ids: Vec<chiral_common_new::data::types::EntryID>,
    fps: Vec<chiral_common_new::app::chem::types::FingerprintData>
}

impl From<(&chiral_common_new::app::chem::kinds::Fingerprint, &chiral_common_new::kinds::Dataset, &chiral_common_new::data::DocSMILES)> for Data {
    fn from((fpk, dsk_in, doc): (&chiral_common_new::app::chem::kinds::Fingerprint, &chiral_common_new::kinds::Dataset, &chiral_common_new::data::DocSMILES)) -> Self {
        let fpk_ob = super::to_ob_fp_kind(fpk);
        let fpg = openbabel::fingerprint::FingerprintGenerator::new(fpk_ob);
        let fps = fpg.get_fingerprint_for_smiles_vec(doc.get_smiles_vec());
        let ids = doc.get_ids().to_vec();
        Self { dsk: dsk_in.to_owned(), ids, fps }
    }
}

impl chiral_common_new::traits::TraitData for Data {
    fn blank() -> Self {
        Self { dsk: chiral_common_new::kinds::Dataset::Empty, ids: vec![], fps: vec![] }
    }

    fn len(&self) -> usize {
        self.ids.len()
    }
}

/// Operator
pub struct Operator {
    fpk: chiral_common_new::app::chem::kinds::Fingerprint,
    fpg: openbabel::fingerprint::FingerprintGenerator,
}

impl From<chiral_common_new::app::chem::kinds::Fingerprint> for Operator {
    fn from(fpk: chiral_common_new::app::chem::kinds::Fingerprint) -> Self {
        let fpk_ob = super::to_ob_fp_kind(&fpk);
        let fpg = openbabel::fingerprint::FingerprintGenerator::new(fpk_ob);
        Self { fpk, fpg }
    }
}

impl chiral_common_new::traits::TraitOperator for Operator {
    type InputType = Input;
    type DataType = Data;
    type OutputType = Output; 
    type ReportType = Report;

    fn compute(&self, input: &Self::InputType, data: &Self::DataType) -> Self::OutputType {
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

    fn report(&self, input: Self::InputType, data: &Self::DataType, output: Self::OutputType) -> Self::ReportType {
        Report {
            input, 
            fpk: self.fpk.to_owned(),
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

impl From<chiral_common_new::app::chem::kinds::Fingerprint> for ComputingUnit {
    fn from(fpk: chiral_common_new::app::chem::kinds::Fingerprint) -> Self {
        let op = Operator::from(fpk);
        Self { op, input: Input::default(), data: Data::blank(), output: Output::blank() }
    }
}

impl chiral_common_new::traits::TraitComputingUnit for ComputingUnit {
    fn set_input(&mut self, input_ser: &chiral_common_new::types::ser::InputSer) {
        self.input = Input::ser_from(input_ser);
    }

    fn set_data(&mut self, dsk: &chiral_common_new::kinds::Dataset, doc: &chiral_common_new::data::DocSMILES) {
        self.data = Data::from((&self.op.fpk, dsk, doc));
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
    fn test_op() {
        let dsk = chiral_common_new::kinds::Dataset::Dummy;
        let doc_smiles = chiral_common_new::data::DocSMILES::dummy();
        let com_fpk = chiral_common_new::app::chem::kinds::Fingerprint::kind_openbabel_ecfp4(2048);
        let op = Operator::from(com_fpk.to_owned());
        let data = Data::from((&com_fpk, &dsk, &doc_smiles));
        assert_eq!(data.len(), 4);
        let input = Input { smiles: String::from("c1ccccc1"), threshold: 0.045 };
        let output = op.compute(&input, &data);
        assert_eq!(output.len(), 2); 
        let report = op.report(input, &data, output);
        let serialized_report = report.ser_to();
        let report_deserialized = Report::ser_from(&serialized_report);
        assert_eq!(report_deserialized.fpk, com_fpk);
        assert_eq!(report_deserialized.dsk, dsk);
        assert_eq!(report_deserialized.output.len(), 2);
    }
}
