//! Fingerprint Kind for OpenBabel

use serde::{Serialize, Deserialize};
    
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]  
pub enum Kind {
    FP2 { nbits: u32 },
    FP3 { nbits: u32 },
    FP4 { nbits: u32 },
    ECFP0 { nbits: u32 },
    ECFP2 { nbits: u32 },
    ECFP4 { nbits: u32 },
    ECFP6 { nbits: u32 },
    ECFP8 { nbits: u32 },
    ECFP10 { nbits: u32 }
}

impl std::default::Default for Kind {
    fn default() -> Self {
        Self::ECFP4 { nbits: 2048 }
    }
}