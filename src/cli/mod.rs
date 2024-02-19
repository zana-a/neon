use clap::Arg;
use clap::ArgMatches;
use clap::Command;

use crate::repl;

const BANNER: &str = r#"
::::    ::: :::::::::: ::::::::  ::::    :::
:+:+:   :+: :+:       :+:    :+: :+:+:   :+:
:+:+:+  +:+ +:+       +:+    +:+ :+:+:+  +:+
+#+ +:+ +#+ +#++:++#  +#+    +:+ +#+ +:+ +#+
+#+  +#+#+# +#+       +#+    +#+ +#+  +#+#+#
#+#   #+#+# #+#       #+#    #+# #+#   #+#+#
###    #### ########## ########  ###    ####
  "#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const NAME: &str = env!("CARGO_PKG_NAME");

pub fn run() {
  let matches = Command::new("Neon")
    .about(create_title())
    .arg(create_version_argument())
    .subcommand(create_new_command())
    .subcommand(create_init_command())
    .subcommand(create_repl_command())
    .arg_required_else_help(true)
    .get_matches();

  handle_version_argument(&matches);
  handle_new_command(&matches);
  handle_init_command(&matches);
  handle_repl_command(&matches);
}

fn create_version_argument() -> Arg {
  Arg::new("version")
    .long("version")
    .short('v')
    .help("Print version info and exit")
    .num_args(0)
}

fn handle_version_argument(matches: &ArgMatches) {
  if matches.get_flag("version") {
    println!("{}", create_name_and_version());
  }
}

fn create_name_and_version() -> String {
  format!("{} {}", NAME, VERSION)
}

fn create_banner() -> String {
  BANNER.trim().to_string()
}

fn create_title() -> String {
  let banner = create_banner();
  let name_and_version = create_name_and_version();

  let banner_length = banner
    .lines()
    .collect::<Vec<&str>>()
    .first()
    .unwrap()
    .len();
  let name_and_version_length = name_and_version.len();

  let spacing = " ".repeat(banner_length - name_and_version_length);

  format!("{}\n{}{}", banner, spacing, name_and_version)
}

fn create_new_command() -> Command {
  Command::new("new").about("Create a new package")
}

fn handle_new_command(matches: &ArgMatches) {
  if let Some(_argument) = matches.subcommand_matches("new") {
    println!("This is the `new` command.");
    println!("Use it to create a new package by providing the `directory` you wish to populate.");
  }
}

fn create_init_command() -> Command {
  Command::new("init").about("Generate a new package in current directory")
}

fn handle_init_command(matches: &ArgMatches) {
  if let Some(_argument) = matches.subcommand_matches("init") {
    println!("This is the `init` command.");
    println!("Use it to create a new package in the current directory. Make sure its empty!");
    println!("Otherwise use --force to ignore overwriting warning.");
  }
}

fn create_repl_command() -> Command {
  Command::new("repl").about("Start a REPL session")
}

fn handle_repl_command(matches: &ArgMatches) {
  if let Some(_argument) = matches.subcommand_matches("repl") {
    repl::run();
  }
}
