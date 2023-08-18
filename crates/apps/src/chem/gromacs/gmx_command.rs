//! Run Gromacs gxm commands
//! 

use chiral_common::apps::chem::gromacs::gmx_command::*;
use chiral_common::traits::*;

/// Data
pub struct Data {}

impl TraitData for Data {
    fn blank() -> Self { Self {} }
    fn len(&self) -> usize { 0 }
}

/// Operator
pub struct Operator {
    home_dir: std::path::PathBuf,
}

impl chiral_common::traits::TraitOperator for Operator {
    type InputType = Input;
    type DataType = Data;
    type OutputType = Output; 
    // type ReportType = Report;

    fn new(opk: &chiral_common::kinds::Operator) -> Self {
        match opk {
            chiral_common::kinds::Operator::GromacsRunGMXCommand => {
                let dir_str = chiral_common::apps::env_var::Variable::ChiralTmpDir.get();
                let home_dir = std::path::PathBuf::from(dir_str).join("gromacs");
                Self { home_dir }
            },
            _ => panic!("Operator Kind not match")
        }
    }

    fn get_kind(&self) -> chiral_common::kinds::Operator {
        chiral_common::kinds::Operator::GromacsRunGMXCommand
    }

    fn prepare_data(&self, _dsk: &chiral_common::kinds::Dataset, _div_index: &chiral_common::job::DividendIndex, _ds: std::sync::Arc<std::sync::Mutex<dyn TraitDataStore>>) -> Option<Self::DataType> {
        Some(Data {})
    }

    fn compute(&self, input: &Self::InputType, _data: &Self::DataType, _div_index: &chiral_common::job::DividendIndex) -> anyhow::Result<Self::OutputType> {
        let dir = self.home_dir.join(&input.simulation_id);
        match gromacs::run_gmx_command(
            input.sub_command.as_str(),
            &input.arguments,
            &dir,
            &input.prompts
        ) {
            Ok(command_outputs) => {
                if command_outputs.is_success() {
                    Ok(Output {
                        stdout: command_outputs.get_stdout_string(),
                        stderr: "".to_string()
                    })
                } else {
                    Err(anyhow::Error::msg(command_outputs.get_stderr_string()))
                }
            },
            Err(e) => Err(anyhow::Error::msg(e.to_string()))
        }
    }

    // fn report(&self, job_id: chiral_common::types::JobID, input: Self::InputType, _data: &Self::DataType, output: Self::OutputType) -> Self::ReportType {
    //     Report {
    //         job_id,
    //         cuk: chiral_common::kinds::ComputingUnit::new(self.get_kind(), chiral_common::kinds::Dataset::Empty),
    //         input, 
    //         output
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gmx_command_op_failed() {
        let chiral_data_dir = chiral_common::apps::env_var::Variable::ChiralDataDir.get();
        let home_dir = "test_op_failed";
        if std::path::PathBuf::from(home_dir).exists() {
            std::fs::remove_dir_all(home_dir).unwrap();
        }
        std::fs::create_dir(home_dir).unwrap();
        std::fs::create_dir(std::path::PathBuf::from(home_dir).join("lysozyme")).unwrap();

        // no input file
        let input = Input {
            simulation_id: "lysozyme".to_string(),
            sub_command: "grompp".to_string(),
            arguments: vec!["-f", "npt.mdp", "-c", "nvt.gro"].iter().map(|s| s.to_string()).collect(),
            prompts: vec![],
            files_dir: "".to_string(),
            files_input: vec![],
            files_output: vec![]
        };
        let data = Data {};
        let op = Operator { home_dir: std::path::PathBuf::from(home_dir) };
        let result = op.compute(&input, &data, &(0, 1));
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("File 'npt.mdp' does not exist"));

        // subcommand error
        std::fs::copy(
            std::path::PathBuf::from(chiral_data_dir).join("gromacs/lysozyme/1AKI_clean.pdb"),
            std::path::PathBuf::from(home_dir).join("lysozyme").join("1AKI_clean.pdb")
        ).unwrap();
        let input = Input { 
            simulation_id: "lysozyme".to_string(),
            sub_command: "pdb2".to_string(), // should be pdb2gmx
            arguments: ["-f", "1AKI_clean.pdb", "-o", "1AKI_processed.gro", "-water", "spce"].iter().map(|s| s.to_string()).collect(),
            prompts: ["15 0"].iter().map(|s| s.to_string()).collect(),
            files_dir: ".".to_string(),
            files_input: ["1AKI_clean.pdb"].iter().map(|s| s.to_string()).collect(),
            files_output: ["1AKI_processed.gro", "topol.top", "posre.itp"].iter().map(|s| s.to_string()).collect(),
        };
        let data = Data {};
        let op = Operator { home_dir: std::path::PathBuf::from(home_dir) };
        let result = op.compute(&input, &data, &(0, 1));
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("'pdb2' is not a GROMACS command"));

        std::fs::remove_dir_all(home_dir).unwrap();
    }

    #[test]
    fn test_gmx_command_op() {
        let chiral_data_dir = chiral_common::apps::env_var::Variable::ChiralDataDir.get();
        let home_dir = "test_op";
        if std::path::PathBuf::from(home_dir).exists() {
            std::fs::remove_dir_all(home_dir).unwrap();
        }
        std::fs::create_dir(home_dir).unwrap();
        std::fs::create_dir(std::path::PathBuf::from(home_dir).join("lysozyme")).unwrap();

        std::fs::copy(
            std::path::PathBuf::from(chiral_data_dir).join("gromacs/lysozyme/1AKI_clean.pdb"),
            std::path::PathBuf::from(home_dir).join("lysozyme").join("1AKI_clean.pdb")
        ).unwrap();
        let input = Input::default(); 
        let data = Data {};
        for fi in input.files_in().iter() {
            let fip = std::path::PathBuf::from(home_dir).join(&input.simulation_id).join(fi);
            assert!(fip.exists());
        }
        let op = Operator { home_dir: std::path::PathBuf::from(home_dir) };
        let result = op.compute(&input, &data, &(0, 1));
        assert!(result.is_ok());
        assert!(result.unwrap().stdout.len() > 0);
        for fo in input.files_out().iter() {
            let fop = std::path::PathBuf::from(home_dir).join(&input.simulation_id).join(fo);
            assert!(fop.exists());
            std::fs::remove_file(fop).unwrap();
        }
        
        std::fs::remove_dir_all(home_dir).unwrap();
    }
}