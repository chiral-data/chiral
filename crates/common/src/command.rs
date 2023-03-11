//! Common stuff for CLI
//! 

use thiserror::Error;

pub const HELP_TEMPLATE: &str = "\
    {about-with-newline}\n\
    {usage-heading}\n    {usage}\n\
    \n\
    {all-args}{after-help}\
";


#[derive(Error, Debug)]
pub enum CommandLineError {
    #[error("shlex split argument error: invalid quoting")]
    ShlexSplitError,
    #[error("subcommand '{0} {1}' not found, run '{0} help' for details.")]
    SubCommandNotFound(String, String),
    #[error("subcommand required after'{0}', run '{0} help' for details.")]
    SubCommandRequired(String),
    #[error("argument '{0}' not found.")] 
    ArgumentNotFound(String),
}