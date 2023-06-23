use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::*;

use crate::prelude::meta::*;

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
        Command::new(Self::COMMAND_NEW)
            .about("Create a new neon package")
            .arg(
                Arg::new("path")
                    .required(true)
                    .action(ArgAction::Set)
                    .help("The path location of the new project"),
            )
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

    // TODO: This needs fixing. Too clumsy
    fn create_boilerplate_project(path: &Path) -> std::io::Result<()> {
        create_dir_all(path.join("src"))?;
        File::create(path.join("src").join("main.ne"))?.write_all(b"buffer")?;
        File::create(path.join("Neon.yaml"))?.write_all(b"buffer")?;
        Ok(())
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
