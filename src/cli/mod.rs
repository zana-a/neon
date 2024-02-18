use clap::ArgMatches;
use clap::Command;

const HEADER: &str = r#"
::::    ::: :::::::::: ::::::::  ::::    :::
:+:+:   :+: :+:       :+:    :+: :+:+:   :+:
:+:+:+  +:+ +:+       +:+    +:+ :+:+:+  +:+
+#+ +:+ +#+ +#++:++#  +#+    +:+ +#+ +:+ +#+
+#+  +#+#+# +#+       +#+    +#+ +#+  +#+#+#
#+#   #+#+# #+#       #+#    #+# #+#   #+#+#
###    #### ########## ########  ###    ####
  "#;

pub fn run() {
  let matches = Command::new("Neon")
    .about(HEADER.trim())
    .subcommand(create_new_command())
    .subcommand(create_init_command())
    .arg_required_else_help(true)
    .get_matches();

  handle_new_command(&matches);
  handle_init_command(&matches);
}

fn create_new_command() -> Command {
  Command::new("new").about("Create a new Neon package")
}

fn handle_new_command(matches: &ArgMatches) {
  if let Some(_argument) = matches.subcommand_matches("new") {
    println!("This is the `new` command.");
    println!("Use it to create a new Neon package by providing the `directory` you wish to populate.");
  }
}

fn create_init_command() -> Command {
  Command::new("init").about("Generate a new Neon package in current directory")
}

fn handle_init_command(matches: &ArgMatches) {
  if let Some(_argument) = matches.subcommand_matches("init") {
    println!("This is the `init` command.");
    println!("Use it to create a new Neon package in the current directory. Make sure its empty!");
    println!("Otherwise use --force to ignore overwriting warning.");
  }
}
