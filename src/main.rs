use std::io;
use clap::{Arg, ArgAction, ArgMatches, Command, arg};
use std::fs;
use std::path::{Path, PathBuf};

const CSET_PATH: &str = "/sys/fs/cgroup/cpuset";

pub fn app() -> Command {
    Command::new("cset")
        .subcommand(
            Command::new("proc")
                .arg(arg!(-l --list "list"))
        )
        .subcommand(
            Command::new("set")
                .arg(arg!(-l --list [path] "list").default_value("/"))
                .arg(arg!(-r --recursive "recursive"))
                .arg(arg!(-c --cpu <mask> "sets a cpumask"))
        )
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&Path)) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            cb(&path);
            visit_dirs(&path, cb)?;
        }
    }
    Ok(())
}

fn enter_dirs(path : &str, cb : &dyn Fn(&Path), recursive : bool) -> io::Result<()> {
    let path = CSET_PATH.to_owned() + path;
    let path = PathBuf::from(path);
    cb(&path);
    for dir in fs::read_dir(path)? {
        let dir = dir?;
        let path = dir.path();

        if path.is_dir() {
            cb(&path);
            if recursive {
                visit_dirs(&path, cb)?;
            }
        }
    }
    Ok(())
}

fn print_cpuset(entry : &Path) {
    let path = entry.to_str().unwrap().to_owned() + "/cpuset";
    match fs::read_to_string(path) {
        Ok(buf) => println!("{:?} {}", entry, buf.trim()),
        Err(_) => println!("{:?}", entry),
    }
}

fn do_proc(matches : &ArgMatches) -> io::Result<()> {
    println!("proc");
    Ok(())
}

fn do_set(matches : &ArgMatches) -> io::Result<()> {
    if matches.contains_id("list") {
        let mut list = "/";
        if let Some(arg) = matches.get_one::<String>("list") {
            list = arg;
        }
        return enter_dirs(list, &print_cpuset, matches.get_flag("recursive"));
    }

    if let Some(mask) = matches.get_one::<String>("cpu") {
        println!("cpumask : {}", mask);
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
