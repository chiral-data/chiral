pub mod similarity;
pub mod substructure;


/// Convert Fingerprint Kinds
pub fn to_ob_fp_kind(kind: &chiral_common::app::chem::kinds::Fingerprint) -> openbabel::fingerprint::Kind {
    match kind {
        chiral_common::app::chem::kinds::Fingerprint::OpenBabel { kind: common_op_fp_kind } => {
            match common_op_fp_kind {
                chiral_common::app::chem::openbabel::kinds::Fingerprint::FP2 { nbits } => openbabel::fingerprint::Kind::FP2 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::FP3 { nbits } => openbabel::fingerprint::Kind::FP2 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::FP4 { nbits } => openbabel::fingerprint::Kind::FP2 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::ECFP0 { nbits } => openbabel::fingerprint::Kind::ECFP0 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::ECFP2 { nbits } => openbabel::fingerprint::Kind::ECFP2 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::ECFP4 { nbits } => openbabel::fingerprint::Kind::ECFP4 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::ECFP6 { nbits } => openbabel::fingerprint::Kind::ECFP6 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::ECFP8 { nbits } => openbabel::fingerprint::Kind::ECFP8 { nbits: *nbits },
                chiral_common::app::chem::openbabel::kinds::Fingerprint::ECFP10 { nbits } => openbabel::fingerprint::Kind::ECFP10 { nbits: *nbits },
            }
        }
    }
}
