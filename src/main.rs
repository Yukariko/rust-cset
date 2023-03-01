use clap::{Arg, ArgAction, ArgMatches, Command, arg};

pub fn app() -> Command {
    Command::new("cset")
        .subcommand(
            Command::new("proc")
                .arg(arg!(-l --list "list"))
        )
        .subcommand(
            Command::new("set")
                .arg(arg!(-l --list "list"))
        )
}

fn main() {
    let matches = app().get_matches();
}
