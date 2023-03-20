//! Commands for using RecGen in Chiral SaaS
//! 

//! Commands for using OpenBabel in Chiral SaaS 
//! 

use anyhow::Result;
use std::io::Read;
use crate::traits::Serialization;
use crate::command::HELP_TEMPLATE;

const COMMAND: &str = "recgen";

const EXAMPLES: &str = "\
\nReCGen Synopsis
recgen build --input /home/users/sample.mol
";

pub fn set() -> clap::Command {
    clap::Command::new(COMMAND)
        .about("recgen commands")
        .subcommand(
            clap::Command::new("examples")
            .about("show command examples")
        )
        .subcommand(
            clap::Command::new("build")
                .about("run recgen synopsis")
                .arg(
                    clap::Arg::new("input")
                        .long("input")
                        .required(true),
                )
        )
        .help_template(HELP_TEMPLATE)
}

fn input(matches: &clap::ArgMatches) -> Result<String> {
    let input_filepath_str = matches.get_one::<String>("input").ok_or(crate::command::CommandLineError::ArgumentNotFound("input".to_string()))?;
    let filepath = std::path::PathBuf::from(input_filepath_str);
    let file = std::fs::File::open(&filepath)?;
    let mut buf = String::new();
    std::io::BufReader::new(file).read_to_string(&mut buf)?;
    Ok(buf)
}

fn build(matches: &clap::ArgMatches) -> Result<Option<crate::job::Job>> {
    let mol = input(matches)?;
    let input = crate::app::chem::recgen::build::Input { mol }; 
    let req = crate::job::Requirement::new(input.ser_to(), crate::kinds::Operator::ReCGenBuild, crate::kinds::Dataset::Empty);

    Ok(Some(crate::job::Job::new(req)))
}

pub fn parse(matches: &clap::ArgMatches) -> Result<Option<crate::job::Job>> {
    match matches.subcommand() {
        Some(("examples", _matches)) => {
            crate::logging::info(EXAMPLES);
            Ok(None)
        }
        Some(("build", matches)) => build(matches),
        Some((name, _matches)) => Err(crate::command::CommandLineError::SubCommandNotFound(COMMAND.to_string(), name.to_string()).into()),
        None => Err(crate::command::CommandLineError::SubCommandRequired(COMMAND.to_string()).into())
    }
}

#[cfg(test)]
mod tests {
}