use std::io::{self, Write};
use clap::{Arg, ArgAction, ArgMatches, Command, arg};
use std::fs;
use std::path::{Path, PathBuf};

const CSET_PATH: &str = "/sys/fs/cgroup/cpuset";

struct Procedure<'a> {
    pre_cb : &'a dyn Fn(&Path),
    post_cb : &'a dyn Fn(&Path),
    recursive : bool,
}

pub fn app() -> Command {
    Command::new("cset")
        .subcommand(
            Command::new("proc")
                .arg(arg!(-l --list "list"))
        )
        .subcommand(
            Command::new("set")
                .arg(arg!([path] "path").default_value("/"))
                .arg(arg!(-l --list "list"))
                .arg(arg!(-r --recursive "recursive"))
                .arg(arg!(-c --cpu <mask> "sets a cpumask"))
        )
}

fn visit_dirs(dir: &Path, proc : &Procedure) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            (proc.pre_cb)(&path);
            visit_dirs(&path, &proc)?;
            (proc.post_cb)(&path);
        }
    }
    Ok(())
}

fn enter_dirs(path : &str, proc : &Procedure) -> io::Result<()> {
    let path = CSET_PATH.to_owned() + path;
    let path = PathBuf::from(path);

    (proc.pre_cb)(&path);
    for dir in fs::read_dir(&path)? {
        let dir = dir?;
        let path = dir.path();

        if path.is_dir() {
            (proc.pre_cb)(&path);
            if proc.recursive {
                visit_dirs(&path, &proc)?;
            }
            (proc.post_cb)(&path);
        }
    }
    (proc.post_cb)(&path);
    Ok(())
}

fn print_cpuset(entry : &Path) {
    let path = entry.to_str().unwrap().to_owned() + "/cpuset";
    match fs::read_to_string(path) {
        Ok(buf) => println!("{:?} {}", entry, buf.trim()),
        Err(_) => println!("{:?}", entry),
    }
}

fn empty_function(entry : &Path) {

}

fn set_cpuset(entry : &Path, mask : &str) {
    let path = entry.to_str().unwrap().to_owned() + "/cpuset";
    match fs::File::options().write(true).truncate(true).open(path) {
        Ok(mut f) => {
            f.write_all(mask.as_bytes());
            println!("{:?} {}", entry, mask)
        },
        Err(_) => (),
    }
}

fn do_proc(matches : &ArgMatches) -> io::Result<()> {
    println!("proc");
    Ok(())
}

fn do_set(matches : &ArgMatches) -> io::Result<()> {
    let mut path = "/";
    if let Some(arg) = matches.get_one::<String>("path") {
        path = arg;
    }

    if matches.get_flag("list") {
        let proc = Procedure {
            pre_cb : &print_cpuset,
            post_cb : &empty_function,
            recursive : matches.get_flag("recursive"),
        };
        return enter_dirs(path, &proc);
    }

    if let Some(mask) = matches.get_one::<String>("cpu") {
        let proc = Procedure {
            pre_cb : &empty_function,
            post_cb : &|entry : &Path| { set_cpuset(&entry, mask); },
            recursive : matches.get_flag("recursive"),
        };
        return enter_dirs(path, &proc);
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
