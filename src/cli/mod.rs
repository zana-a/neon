use clap::*;

use crate::prelude::meta::*;

pub struct Cli;

impl Cli {
    const COMMAND_NEW: &str = "new";
    const COMMAND_BUILD: &str = "build";
    const COMMAND_CLEAN: &str = "clean";

    pub fn run() {
        let root_command = Command::new(NAME)
            .version(VERSION)
            .author(AUTHORS)
            .about(DESCRIPTION)
            .arg_required_else_help(true)
            .before_help(Self::create_before_help())
            .subcommand(Self::create_new_command())
            .subcommand(Self::create_build_command())
            .subcommand(Self::create_clean_command())
            .get_matches();

        Self::handle_new_command(&root_command);
        Self::handle_build_command(&root_command);
        Self::handle_clean_command(&root_command);
    }

    fn create_before_help() -> String {
        format!("{}\n({})", BANNER, VERSION)
    }

    fn create_new_command() -> Command {
        Command::new(Self::COMMAND_NEW)
            .about("Create a new neon package")
            .arg(arg!(<name>))
    }

    fn create_build_command() -> Command {
        Command::new(Self::COMMAND_BUILD)
            .about("Build a local neon package")
            .arg(arg!(<path>))
    }

    fn create_clean_command() -> Command {
        Command::new(Self::COMMAND_CLEAN)
            .about("Remove artifacts that neon has generated")
            .arg(arg!(<path>))
    }

    fn handle_new_command(root_command: &ArgMatches) {
        if root_command.subcommand_matches(Self::COMMAND_NEW).is_some() {
            println!("Creating new project");
        }
    }

    fn handle_build_command(root_command: &ArgMatches) {
        if root_command
            .subcommand_matches(Self::COMMAND_BUILD)
            .is_some()
        {
            println!("Building project");
        }
    }

    fn handle_clean_command(root_command: &ArgMatches) {
        if root_command
            .subcommand_matches(Self::COMMAND_CLEAN)
            .is_some()
        {
            println!("Cleaning project");
        }
    }
}
