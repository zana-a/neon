use clap::*;

use crate::prelude::meta::*;

pub struct Cli;

impl Cli {
    pub fn run() {
        let root_command = Command::new(NAME)
            .version(VERSION)
            .author(AUTHORS)
            .about(DESCRIPTION)
            .arg_required_else_help(true)
            .before_help(Self::create_before_help())
            .subcommand(Self::create_new_command())
            .subcommand(Self::create_build_command())
            .subcommand(Self::create_clean_command());

        root_command.get_matches();
    }

    fn create_before_help() -> String {
        format!("{}\n({})", BANNER, VERSION)
    }

    fn create_new_command() -> Command {
        Command::new("new")
            .about("Create a new neon package")
            .arg(arg!(<path>))
    }

    fn create_build_command() -> Command {
        Command::new("build")
            .about("Build a local neon package")
            .arg(arg!(<path>))
    }

    fn create_clean_command() -> Command {
        Command::new("clean")
            .about("Remove artifacts that neon has generated")
            .arg(arg!(<path>))
    }
}
