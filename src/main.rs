use clap::{Arg, ArgAction, ArgMatches, Command, arg};
use std::fs;

const CSET_PATH: &str = "/sys/fs/cgroup/cpuset";

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

fn do_proc(matches : &ArgMatches) -> std::io::Result<()> {
    println!("proc");
    Ok(())
}

fn do_set(matches : &ArgMatches) -> std::io::Result<()> {
    for dir in fs::read_dir(CSET_PATH)? {
        let dir = dir?;

        if dir.path().is_dir() {
            if let Some(path) = dir.file_name().to_str() {
                println!("{}", path);
            }
        }
    }
    Ok(())
}

fn main() {
    let matches = app().get_matches();
    let results = match matches.subcommand() {
        Some(("proc", sub_matches)) => do_proc(sub_matches),
        Some(("set", sub_matches)) => do_set(sub_matches),
        _ => Ok(()),
    };
}
