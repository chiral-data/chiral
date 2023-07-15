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
    type ReportType = Report;

    fn new(opk: &chiral_common::kinds::Operator) -> Self {
        match opk {
            chiral_common::kinds::Operator::GromacsRunGMXCommand => {
                let dir_str = chiral_common::apps::env_var::Variable::GromacsWorkDir.get();
                let home_dir = std::path::PathBuf::from(dir_str);
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

    fn compute(&self, input: &Self::InputType, _data: &Self::DataType, _div_index: &chiral_common::job::DividendIndex) -> Self::OutputType {
        let dir = self.home_dir.join(&input.simulation_id);
        if !dir.exists() {
            return Output { success: false, stdout: "".to_string(), stderr: format!("Directory {dir:?} does not exist!") };
        }

        match gromacs::run_gmx_command(
            input.sub_command.as_str(),
            &input.arguments,
            &dir,
            &input.prompts
        ) {
            Ok(command_outputs) => Output {
                success: command_outputs.is_success(),
                stdout: command_outputs.get_stdout_string(),
                stderr: command_outputs.get_stderr_string()
            },
            Err(e) => Output {
                success: false,
                stdout: "".to_string(),
                stderr: e.to_string()
            }
        }
    }

    fn report(&self, job_id: chiral_common::types::JobID, input: Self::InputType, _data: &Self::DataType, output: Self::OutputType) -> Self::ReportType {
        Report {
            job_id,
            cuk: chiral_common::kinds::ComputingUnit::new(self.get_kind(), chiral_common::kinds::Dataset::Empty),
            input, 
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gmx_command_op_failed() {
        let home_dir = std::path::PathBuf::from("../../../gromacs-rust/tutorials");
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
        let op = Operator { home_dir: home_dir.to_owned() };
        let output = op.compute(&input, &data, &(0, 1));
        assert_eq!(output.success, false);
        assert!(output.stdout.len() == 0);
        assert!(output.stderr.contains("File 'npt.mdp' does not exist"));
    }

    #[test]
    fn test_gmx_command_op() {
        let home_dir = std::path::PathBuf::from("../../../gromacs-rust/tutorials");
        let input = Input::default(); 
        let data = Data {};
        for fi in input.files_in().iter() {
            let fip = home_dir.join(&input.simulation_id).join(fi);
            assert!(fip.exists());
        }
        let op = Operator { home_dir: home_dir.to_owned() };
        let output = op.compute(&input, &data, &(0, 1));
        assert_eq!(output.success, true);
        assert!(output.stdout.len() > 0);
        assert!(output.stderr.len() > 0);
        for fo in input.files_out().iter() {
            let fop = home_dir.join(&input.simulation_id).join(fo);
            assert!(fop.exists());
            std::fs::remove_file(fop).unwrap();
        }
    }
}