use std::path::Path;

use clap::*;

use crate::cli::meta::*;

mod generator;
mod meta;
mod runner;

pub struct Cli;

impl Cli {
    const COMMAND_NEW: &'static str = "new";
    const COMMAND_BUILD: &'static str = "build";
    const COMMAND_CLEAN: &'static str = "clean";

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
        let arg = Arg::new("path")
            .required(true)
            .action(ArgAction::Set)
            .help("The path location of the new project");

        Command::new(Self::COMMAND_NEW)
            .about("Create a new neon package")
            .arg(arg)
    }

    fn create_build_command() -> Command {
        Command::new(Self::COMMAND_BUILD).about("Build a local neon package")
    }

    fn create_clean_command() -> Command {
        Command::new(Self::COMMAND_CLEAN).about("Remove artifacts that neon has generated")
    }

    fn handle_new_command(root_command: &ArgMatches) {
        if let Some(command) = root_command.subcommand_matches(Self::COMMAND_NEW) {
            if let Some(path) = command.get_one::<String>("path") {
                match Self::create_boilerplate_project(Path::new(path)) {
                    Ok(_) => println!("Created project at: {}", path),
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }

    fn create_boilerplate_project(path: &Path) -> std::io::Result<()> {
        generator::Generator::create_bin_project(path)
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
