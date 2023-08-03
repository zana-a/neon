use clap::Command;

fn create_title() -> String {
    format!(
        "
::::    ::: :::::::::: ::::::::  ::::    ::: 
:+:+:   :+: :+:       :+:    :+: :+:+:   :+: 
:+:+:+  +:+ +:+       +:+    +:+ :+:+:+  +:+ 
+#+ +:+ +#+ +#++:++#  +#+    +:+ +#+ +:+ +#+ 
+#+  +#+#+# +#+       +#+    +#+ +#+  +#+#+# 
#+#   #+#+# #+#       #+#    #+# #+#   #+#+# 
###    #### ########## ########  ###    #### 
v{}

Neon's package manager
    ",
        env!("CARGO_PKG_VERSION")
    )
}

pub fn run() {
    Command::new("Neon")
        .about(create_title().trim().to_owned())
        .subcommand(Command::new("new").about("Generate a new Neon package"))
        .arg_required_else_help(true)
        .get_matches();
}
