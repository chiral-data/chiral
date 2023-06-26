use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};
use crate::traits::*;

#[derive(EnumString, Display, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub enum Kind {
    #[strum(serialize = "ob_sim")]
    OpenBabelSimilaritySearching(crate::app::chem::kinds::Fingerprint),
    #[strum(serialize = "ob_ss")]
    OpenBabelSSMatching,
    #[strum(serialize = "recgen_build")]
    ReCGenBuild,
    #[strum(serialize = "gromacs_run_gmx_command")]
    GromacsRunGMXCommand
}

impl Kind {
    pub fn default_ob_similarity_searching() -> Self {
        let fpk = crate::app::chem::kinds::Fingerprint::kind_openbabel_ecfp4(2048);
        Kind::OpenBabelSimilaritySearching(fpk)
    }

    pub fn is_openbabel(&self) -> bool {
        match self {
            Self::OpenBabelSSMatching | Self::OpenBabelSimilaritySearching(_) => true, 
            _ => false
        }
    }

    pub fn report_print(&self, content: &crate::traits::SerializedFormat) {
        match self {
            crate::kinds::Operator::OpenBabelSimilaritySearching(_) => super::chem::openbabel::similarity::Report::ser_from(content).print(),
            crate::kinds::Operator::OpenBabelSSMatching => super::chem::openbabel::substructure::Report::ser_from(content).print(),
            crate::kinds::Operator::ReCGenBuild => super::chem::recgen::build::Report::ser_from(content).print(),
            _ => unimplemented!("not implemented") 
        }
    }

    pub fn report_save(&self, job_id: crate::job::ID, dsk: crate::kinds::Dataset, input_ser: &crate::traits::SerializedFormat, output_sers: &Vec<crate::traits::SerializedFormat>, filepath: &std::path::PathBuf) -> std::io::Result<u64> {
        let cuk = crate::kinds::ComputingUnit::new(self.to_owned(), dsk); 
        match self {
            Kind::OpenBabelSimilaritySearching(_) => super::chem::openbabel::similarity::Report::new((job_id, cuk, input_ser, output_sers)).save(filepath),
            Kind::OpenBabelSSMatching => super::chem::openbabel::substructure::Report::new((job_id, cuk, input_ser, output_sers)).save(filepath),
            Kind::ReCGenBuild => super::chem::recgen::build::Report::new((job_id, cuk, input_ser, output_sers)).save(filepath),
            _ => unimplemented!("not implemented") 
        }
    }

    pub fn app_name(&self) -> &str {
        match self {
            Self::OpenBabelSSMatching | Self::OpenBabelSimilaritySearching(_) => "openbabel",
            Self::ReCGenBuild => "my_presto",
            Self::GromacsRunGMXCommand => "gromacs"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn test_strum() {
        // assert_eq!(Kind::from_str("ob_sim").unwrap(), Kind::OpenBabelSimilaritySearching);
        assert_eq!(Kind::OpenBabelSimilaritySearching(crate::app::chem::kinds::Fingerprint::kind_openbabel_ecfp4(1024)).to_string(), "ob_sim");
        assert_eq!(Kind::from_str("ob_ss").unwrap(), Kind::OpenBabelSSMatching);
        assert_eq!(Kind::OpenBabelSSMatching.to_string(), "ob_ss");
    }
}