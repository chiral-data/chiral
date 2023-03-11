//! Fingerprint Kind

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug)]
pub enum ParseStringError {
    NotThreeParts(String),
    NotIntegerNbits(String),
    TypeNotFound((String, String))
}

impl std::error::Error for ParseStringError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ParseStringError::NotThreeParts(_) => None,
            ParseStringError::NotIntegerNbits(_) => None,
            ParseStringError::TypeNotFound(_) => None
        }
    }
}

impl std::fmt::Display for ParseStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotThreeParts(input_str) => format!("Input str {} shall be in format (package)_(fingperint)_(nbits), eg. ob_ecfp2_512", input_str).fmt(f),
            Self::NotIntegerNbits(nbits_str) => format!("str {} cannot be integer for parameter nbits", nbits_str).fmt(f),
            Self::TypeNotFound((pkg_str, fp_str)) => format!("Cannot find package {} or fingerprint  {}", pkg_str, fp_str).fmt(f)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]  
pub enum Kind {
    OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint }
}

impl std::default::Default for Kind {
    fn default() -> Self {
        Self::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::default() }
    }
}

impl Kind {
    pub fn kind_openbabel_fp2(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::FP2 { nbits } } }
    pub fn kind_openbabel_fp3(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::FP3 { nbits } } }
    pub fn kind_openbabel_fp4(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::FP4 { nbits } } }
    pub fn kind_openbabel_ecfp0(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::ECFP0 { nbits } } }
    pub fn kind_openbabel_ecfp2(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::ECFP2 { nbits } } }
    pub fn kind_openbabel_ecfp4(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::ECFP4 { nbits } } }
    pub fn kind_openbabel_ecfp6(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::ECFP6 { nbits } } }
    pub fn kind_openbabel_ecfp8(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::ECFP8 { nbits } } }
    pub fn kind_openbabel_ecfp10(nbits: u32) -> Kind { Kind::OpenBabel { kind: crate::app::chem::openbabel::kinds::Fingerprint::ECFP10 { nbits } } }

    pub fn new(pkg_str: &str, fp_str: &str, nbits: u32) -> Result<Self, ParseStringError> {
        match (pkg_str, fp_str) {
            ("ob", "fp2") => Ok(Self::kind_openbabel_fp2(nbits)),
            ("ob", "fp3") => Ok(Self::kind_openbabel_fp3(nbits)),
            ("ob", "fp4") => Ok(Self::kind_openbabel_fp4(nbits)),
            ("ob", "ecfp0") => Ok(Self::kind_openbabel_ecfp0(nbits)),
            ("ob", "ecfp2") => Ok(Self::kind_openbabel_ecfp2(nbits)),
            ("ob", "ecfp4") => Ok(Self::kind_openbabel_ecfp4(nbits)),
            ("ob", "ecfp6") => Ok(Self::kind_openbabel_ecfp6(nbits)),
            ("ob", "ecfp8") => Ok(Self::kind_openbabel_ecfp8(nbits)),
            ("ob", "ecfp10") => Ok(Self::kind_openbabel_ecfp10(nbits)),
            _ => Err(ParseStringError::TypeNotFound((pkg_str.to_string(), fp_str.to_string())))
        } 
    }
}

impl std::str::FromStr for Kind {
    type Err = ParseStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("_").collect::<Vec<&str>>();
        if parts.len() != 3 { return Err(ParseStringError::NotThreeParts(s.to_string())) }

        let nbits: u32 = parts[2].to_string().parse().or(Err(ParseStringError::NotIntegerNbits(parts[2].to_string())))?;
        Self::new(parts[0], parts[1], nbits)
    }
}

impl std::string::ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Self::OpenBabel { kind } => {
                match kind {
                    crate::app::chem::openbabel::kinds::Fingerprint::FP2 { nbits } => format!("ob_fp2_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::FP3 { nbits } => format!("ob_fp3_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::FP4 { nbits } => format!("ob_fp4_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::ECFP0 { nbits } => format!("ob_ecfp0_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::ECFP2 { nbits } => format!("ob_ecfp2_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::ECFP4 { nbits } => format!("ob_ecfp4_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::ECFP6 { nbits } => format!("ob_ecfp6_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::ECFP8 { nbits } => format!("ob_ecfp8_{}", nbits),
                    crate::app::chem::openbabel::kinds::Fingerprint::ECFP10 { nbits } => format!("ob_ecfp10_{}", nbits),
                }
            }
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
        let fp1 = Kind::from_str("ob_fp4_512");
        assert_eq!(fp1, Ok(Kind::kind_openbabel_fp4(512)));
        assert_eq!(fp1.unwrap().to_string(), "ob_fp4_512");
        let fp2 = Kind::from_str("ob_fp4");
        assert_eq!(fp2, Err(ParseStringError::NotThreeParts("ob_fp4".to_string())));
        let fp3 = Kind::from_str("ob_fp4_51k");
        assert_eq!(fp3, Err(ParseStringError::NotIntegerNbits("51k".to_string())));
        let fp4 = Kind::from_str("rdkit_fp4_512");
        assert_eq!(fp4, Err(ParseStringError::TypeNotFound(("rdkit".to_string(), "fp4".to_string()))));
        let fp5 = Kind::from_str("ob_fp5_512");
        assert_eq!(fp5, Err(ParseStringError::TypeNotFound(("ob".to_string(), "fp5".to_string()))));
    }
}