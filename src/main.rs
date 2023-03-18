use std::io;
use clap::{Arg, ArgAction, ArgMatches, Command, arg};
use std::path::{Path};
use rust_cset::cpuset::*;

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
                .arg(arg!(-c --cpu <mask> "set cpumask"))
                .arg(arg!(-d --destroy "destroy cpusets"))
        )
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

    let empty = |_entry : &Path| ();

    if matches.get_flag("list") {
        let proc = Procedure {
            pre_cb : &print_cpuset,
            post_cb : &empty,
            recursive : matches.get_flag("recursive"),
        };
        return enter_dirs(path, &proc);
    }

    if let Some(mask) = matches.get_one::<String>("cpu") {
        let proc = Procedure {
            pre_cb : &empty,
            post_cb : &|entry : &Path| { set_cpuset(&entry, mask); },
            recursive : matches.get_flag("recursive"),
        };
        return enter_dirs(path, &proc);
    }

    if matches.get_flag("destroy") {
        let proc = Procedure {
            pre_cb : &empty,
            post_cb : &destroy_cpuset,
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
