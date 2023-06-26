//! Commands for using Gromacs in Chiral Cli
//!     - !!! to be implemented
//!  


pub fn set() -> clap::Command {
    clap::Command::new("gmx")
        .about("gromacs commands")
        .ignore_errors(true)
        .help_template(crate::command::HELP_TEMPLATE)
}

#[allow(dead_code)]
fn gxm_command(_args: &Vec<String>) -> anyhow::Result<Option<crate::job::Job>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let cmd_1 = set();
        let res_1 = cmd_1.try_get_matches_from("gmx genion -s ions.tpr -o 1AKI_solv_ions.gro -p topol.top -pname NA -nname CL -neutral".split(" "));
        assert!(res_1.is_ok());
    }
}