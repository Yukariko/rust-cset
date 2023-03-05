use std::io;
use clap::{Arg, ArgAction, ArgMatches, Command, arg};
use std::fs::{self, DirEntry};
use std::path::Path;

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
                .arg(arg!(-r --recursive "recursive"))
        )
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            cb(&entry);
            visit_dirs(&path, cb)?;
        }
    }
    Ok(())
}

fn enter_dirs(path : &str, cb : &dyn Fn(&DirEntry), recursive : bool) -> io::Result<()> {
    let path = CSET_PATH.to_owned() + path;
    for dir in fs::read_dir(path)? {
        let dir = dir?;
        let path = dir.path();

        if path.is_dir() {
            cb(&dir);
            if recursive {
                visit_dirs(&path, cb);
            }
        }
    }
    Ok(())
}

fn do_proc(matches : &ArgMatches) -> io::Result<()> {
    println!("proc");
    Ok(())
}

fn do_set(matches : &ArgMatches) -> io::Result<()> {
    if matches.get_flag("list") {
        println!("list");
        enter_dirs("/", &|e| println!("{:?}", e), matches.get_flag("recursive"));
        return Ok(())
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
