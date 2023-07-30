//! Types used in applications 
//! 
//! 

pub type EntryID = String;
// Chemistry
pub type SMILES = String;
pub type SMARTS = String;
pub type FingerprintData = Vec<u32>;
pub type IdSmilesPairs = (Vec<EntryID>, Vec<SMILES>);