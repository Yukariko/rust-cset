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

fn do_proc(matches : &ArgMatches) {
    println!("proc");
}

fn do_set(matches : &ArgMatches) {
    println!("set");
}

fn main() {
    let matches = app().get_matches();
    match matches.subcommand() {
        Some(("proc", sub_matches)) => do_proc(sub_matches),
        Some(("set", sub_matches)) => do_set(sub_matches),
        _ => println!("none"),
    }
}
