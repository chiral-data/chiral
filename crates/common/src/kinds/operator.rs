use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};

#[derive(EnumString, Display, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub enum Kind {
    #[strum(serialize = "ob_sim")]
    OpenBabelSimilaritySearching(crate::app::chem::kinds::Fingerprint),
    #[strum(serialize = "ob_ss")]
    OpenBabelSSMatching,
    #[strum(serialize = "recgen_build")]
    ReCGenBuilding
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