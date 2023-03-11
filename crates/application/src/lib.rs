pub mod chem;


pub fn create_cu(opk: &chiral_common_new::kinds::Operator) -> Box<dyn chiral_common_new::traits::TraitComputingUnit> {
    match opk {
        chiral_common_new::kinds::Operator::OpenBabelSimilaritySearching(fpk) => Box::new(chem::openbabel::similarity::ComputingUnit::from(fpk.to_owned())),
        chiral_common_new::kinds::Operator::OpenBabelSSMatching => Box::new(chem::openbabel::substructure::ComputingUnit::new()),
        chiral_common_new::kinds::Operator::ReCGenBuilding => unimplemented!() 
    }
}