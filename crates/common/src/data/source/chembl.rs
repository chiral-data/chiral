//! Database ChEMBL
//! https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/
//! 

use std::io::prelude::*;
use rand::prelude::*;

type ChemblID = String;
type CanonicalSMILES = String;
type StandardInchi = String;
type StandardInchiKey = String;

#[derive(PartialEq, Debug)]
pub struct EntryChembl {
    pub chembl_id: ChemblID,
    pub smiles: CanonicalSMILES,
    pub inchi: StandardInchi,
    pub inchi_key: StandardInchiKey
}

impl EntryChembl {
    pub fn new(v: Vec<&str>) -> Self {
        let (chembl_id, smiles, inchi, inchi_key) = (String::from(v[0]), String::from(v[1]), String::from(v[2]), String::from(v[3]));
        Self { chembl_id, smiles, inchi, inchi_key }
    }
}

type DataChembl = std::collections::HashMap<ChemblID, EntryChembl>;

pub struct SourceChembl {
    path: std::path::PathBuf,
    data: DataChembl
}

impl std::default::Default for SourceChembl {
    fn default() -> Self {
        let mut sc = Self::new();
        let kind = crate::kinds::Dataset::Chembl30;
        let chembl_txt = std::env::var_os(kind.env_key()).expect(format!("{} not set", kind.env_key()).as_str());
        sc.set_path(&chembl_txt);
        sc.load_all();
        sc
    }
}

impl SourceChembl {
    pub fn new() -> Self {
        Self {
            path: std::path::PathBuf::new(),
            data: DataChembl::new(),
        }
    }

    pub fn set_path(&mut self, path_str: &std::ffi::OsStr) {
        self.path = std::path::PathBuf::from(path_str)
    }


    fn sanitize(&mut self) {
        self.data.remove("chembl_id");
    }

    fn convert_lines(&mut self, lines: impl std::iter::Iterator<Item = std::io::Result<String>>) {
        self.data.clear();
        self.data = lines.map(|l| {
                let line = l.unwrap();
                // let v = line.as_str().split_whitespace().collect::<Vec<&str>>();
                let v = line.as_str().split("\t").collect::<Vec<&str>>(); // whitespace is not working for some entries like 1077164, inchi is blank
                (String::from(v[0]), EntryChembl::new(v))
            }
        )
        .collect::<Vec<(ChemblID, EntryChembl)>>()
        .into_iter()
        .collect();

        self.sanitize();
    }

    pub fn load_all(&mut self) {
        match std::fs::File::open(&self.path) {
            Ok(file) => {
                let lines = std::io::BufReader::new(file).lines();
                self.convert_lines(lines);
            },
            Err(e) => crate::logging::error(format!("Error {} on file path: {:?}", e, self.path).as_str())
        }
    }

    pub fn load_partial(&mut self, range: &std::ops::Range<usize>) {
        let file = std::fs::File::open(&self.path).unwrap();
        let reader = std::io::BufReader::new(file);
        self.convert_lines(reader.lines().skip(range.start).take(range.len()));
    }

    pub fn get(&self, id: &ChemblID) -> Option<&EntryChembl> { self.data.get(id) }
    pub fn get_all(&self) -> &DataChembl { &self.data }
    pub fn len(&self) -> usize { self.data.len() }

    pub fn get_smiles_id_pairs(&self) -> (Vec<String>, Vec<String>) {
        (
            self.data.values()
                .map(|ec| ec.smiles.clone())
                .collect(),
            self.data.keys()
                .map(|id| id.clone())
                .collect()
        )
    }

    pub fn choices(&self, size: usize) -> Vec<&EntryChembl> {
        let mut rng = thread_rng();
        let marks: Vec<bool> = (0..self.len())
            .map(|_| rng.gen_range(0..self.len()) <= size * 2 )
            .collect();

        self.data.values().enumerate()
            .filter(|(idx, _)| marks[*idx])
            .map(|(_, v)| v)
            .take(size)
            .collect()
    }
}

#[cfg(test)]
mod test_chembl {
    use super::*;

    #[test]
    fn test_source_chembl() {
        let mut sc = SourceChembl::new();
        sc.set_path(&std::ffi::OsString::from("../../../chiral-db-example-data/ChEMBL/chembl_30_chemreps_100.txt"));

        // full load
        sc.load_all();
        assert_eq!(sc.len(), 100);
        let ec = sc.get(&String::from("CHEMBL503634")).unwrap();
        assert_eq!(ec.smiles, "COc1c(O)cc(O)c(C(=N)Cc2ccc(O)cc2)c1O");    
        assert_eq!(ec.inchi, "InChI=1S/C15H15NO5/c1-21-15-12(19)7-11(18)13(14(15)20)10(16)6-8-2-4-9(17)5-3-8/h2-5,7,16-20H,6H2,1H3");
        assert_eq!(ec.inchi_key, "OPELSESCRGGKAM-UHFFFAOYSA-N");
        let data_all = sc.get_all();
        assert_eq!(data_all.keys().count(), 100);
        let selected = sc.choices(10);
        assert_eq!(selected.len(), 10);
        // partial load
        sc.load_partial(&(30..40));
        assert_eq!(sc.len(), 10);
        assert_eq!(sc.get(&String::from("CHEMBL503634")), None);
        let ec = sc.get(&String::from("CHEMBL501923")).unwrap();
        assert_eq!(ec.smiles, "CC(C)=CCC/C(C)=C/Cc1c2c(c3oc4c(c(=O)c3c1O)CC1c3c(c(O)cc(O)c3-4)OC1(C)C)C=CC(C)(C)O2");    
        assert_eq!(ec.inchi, "InChI=1S/C35H38O7/c1-17(2)9-8-10-18(3)11-12-19-28(38)27-29(39)21-15-22-25-26(23(36)16-24(37)33(25)42-35(22,6)7)32(21)40-31(27)20-13-14-34(4,5)41-30(19)20/h9,11,13-14,16,22,36-38H,8,10,12,15H2,1-7H3/b18-11+");
        assert_eq!(ec.inchi_key, "UJHMTIUPFDVYQA-WOJGMQOQSA-N");
    }
}