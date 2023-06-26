//! Commands for using OpenBabel in Chiral Cli 
//! 

use anyhow::{Result, Context};
use std::str::FromStr;
use crate::traits::Serialization;

const EXAMPLES: &str = "\
\nRun OpenBabel substructure matching with input molecule 'c1cccc1N=O' on dataset 'test_chembl'
ob ss --dataset test_chembl --smarts c1ccccc1N=O
\nRun fingerprint based similarity search with input molecule 'c1cccc1N=O' on dataset 'test_chembl', using OpenBabel ECFP4 fingerprint, setting minimal tanimoto coefficient to 0.25
ob sim --dataset test_chembl --smiles c1ccccc1N=O --fingerprint ob_ecfp4_1024 --threshold 0.25
";

pub fn set() -> clap::Command {
    clap::Command::new("ob")
        .about("openbabel commands")
        .subcommand(
            clap::Command::new("examples")
            .about("show command examples")
        )
        .subcommand(
            clap::Command::new("ss")
                .about("run substructure searching")
                .arg(
                    clap::Arg::new("dataset")
                        .long("dataset")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("smarts")
                        .long("smarts")
                        .required(true),
                )
        )
        .subcommand(
            clap::Command::new("sim")
                .about("run similarity searching")
                .arg(
                    clap::Arg::new("dataset")
                        .long("dataset")
                        .required(true),
                )
               .arg(
                    clap::Arg::new("smiles")
                        .long("smiles")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("fingerprint")
                        .long("fingerprint")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("threshold")
                        .long("threshold")
                        .required(true),
                )
        )
        .help_template(crate::command::HELP_TEMPLATE)
}

fn dataset(matches: &clap::ArgMatches) -> Result<crate::kinds::Dataset> {
    let dsk_string = matches.get_one::<String>("dataset").ok_or(crate::command::CommandLineError::ArgumentNotFound("dataset".to_string()))?;
    crate::kinds::Dataset::from_str(dsk_string).map_err(|e| e.into())
}

fn smiles(matches: &clap::ArgMatches) -> Result<&crate::app::chem::types::SMILES> {
    matches.get_one::<String>("smiles").ok_or(crate::command::CommandLineError::ArgumentNotFound("smiles".to_string()).into())
}

fn smarts(matches: &clap::ArgMatches) -> Result<&crate::app::chem::types::SMARTS> {
    matches.get_one::<String>("smarts").ok_or(crate::command::CommandLineError::ArgumentNotFound("smiles".to_string()).into())
}

fn threshold(matches: &clap::ArgMatches) -> Result<f32> {
    let th_str = matches.get_one::<String>("threshold").ok_or(crate::command::CommandLineError::ArgumentNotFound("threshold".to_string()))?;
    th_str.parse::<f32>().context("OpenBabel similarity search - get argument threshold error: ")
}

fn fingerprint(matches: &clap::ArgMatches) -> Result<crate::app::chem::kinds::Fingerprint> {
    let fpk_string = matches.get_one::<String>("fingerprint").ok_or(crate::command::CommandLineError::ArgumentNotFound("fingerprint".to_string()))?;
    crate::app::chem::kinds::Fingerprint::from_str(fpk_string)
        .map_err(|e| e.into())
}

fn similarity(matches: &clap::ArgMatches) -> Result<Option<crate::job::Job>> {
    let dsk = dataset(matches)?;
    let smiles = smiles(matches)?.to_string();
    let fpk = fingerprint(matches)?;
    let threshold = threshold(matches)?;

    let input = crate::app::chem::openbabel::similarity::Input { smiles, threshold };
    let req = crate::job::Requirement::new(input.ser_to(), crate::kinds::Operator::OpenBabelSimilaritySearching(fpk), dsk);

    Ok(Some(crate::job::Job::new(req)))
}

fn substructure(matches: &clap::ArgMatches) -> Result<Option<crate::job::Job>> {
    let dsk = dataset(matches)?;
    let smarts = smarts(matches)?.to_string();

    let input = crate::app::chem::openbabel::substructure::Input { smarts };
    let req = crate::job::Requirement::new(input.ser_to(), crate::kinds::Operator::OpenBabelSSMatching, dsk);

    Ok(Some(crate::job::Job::new(req)))
}

pub fn parse(matches: &clap::ArgMatches) -> Result<Option<crate::job::Job>> {
    match matches.subcommand() {
        Some(("examples", _matches)) => {
            crate::logging::info(EXAMPLES);
            Ok(None)
        }
        Some(("sim", matches)) => similarity(matches),
        Some(("ss", matches)) => substructure(matches),
        Some((name, _matches)) => Err(crate::command::CommandLineError::SubCommandNotFound("ob".to_string(), name.to_string()).into()),
        None => Err(crate::command::CommandLineError::SubCommandRequired("ob".to_string()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let cmd_1 = set();
        let res_1 = cmd_1.try_get_matches_from(vec!["ob", "ss", "--dataset", "hello", "--smarts", "c1ccccc1"]);
        assert!(res_1.is_ok());
        let cmd_2 = set();
        let res_2 = cmd_2.try_get_matches_from(vec!["ob", "ss", "--dataset", "hello"]); 
        assert_eq!(res_2.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
        let cmd_3 = set();
        let res_3 = cmd_3.try_get_matches_from(vec!["ob", "sim", "--dataset", "hello", "--fingerprint", "ob_ecfp4_512"]);
        assert_eq!(res_3.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
        let cmd_4 = set();
        let res_4 = cmd_4.try_get_matches_from(vec!["ob", "sim", "--dataset", "hello", "--fingerprint", "ob_ecfp4_512", "--smiles", "c1cccccc1", "--threshold", "0.1"]);
        assert!(res_4.is_ok());
        if let Ok(matches) = res_4 {
            match matches.subcommand() {
                Some(("sim", matches)) => {
                    let fp_name = matches.get_one::<String>("fingerprint").unwrap();
                    assert_eq!(fp_name.to_string().as_str(), "ob_ecfp4_512");
                },
                _ => {
                    assert!(false);
                }
            }
        }
        let cmd_5 = set();
        let res_5 = cmd_5.try_get_matches_from(vec!["ob", "ss", "--dataset", "hello", "--smarts", "c1ccccc1"]);
        assert!(res_5.is_ok());
        if let Ok(matches) = res_5 {
            match matches.subcommand() {
                Some(("ss", matches)) => {
                    let smarts = matches.get_one::<String>("smarts").unwrap();
                    assert_eq!(smarts.to_string().as_str(), "c1ccccc1");
                },
                _ => {
                    assert!(false);
                }
            }
        }
        let cmd_6 = set();
        let res_6 = cmd_6.try_get_matches_from(vec!["ob", "examples"]);
        assert!(res_6.is_ok());
    }
}