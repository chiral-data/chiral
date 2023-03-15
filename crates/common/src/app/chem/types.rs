pub type SMILES = String;
pub type SMARTS = String;
// Fingerprint Data 
pub type FingerprintData = Vec<u32>;
pub type IdSmilesPairs = (Vec<crate::data::types::EntryID>, Vec<SMILES>);