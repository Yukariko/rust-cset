pub mod cpuset {
    use std::io::{self, Write};
    use std::fs;
    use std::path::{Path, PathBuf};

    const CSET_PATH: &str = "/sys/fs/cgroup/cpuset";

    pub struct Procedure<'a> {
        pub pre_cb : &'a dyn Fn(&Path),
        pub post_cb : &'a dyn Fn(&Path),
        pub recursive : bool,
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

    pub fn enter_dirs(path : &str, proc : &Procedure) -> io::Result<()> {
        let path = CSET_PATH.to_owned() + path;
        let path = PathBuf::from(path);

        (proc.pre_cb)(&path);
        for dir in fs::read_dir(&path)? {
            let dir = dir?;
            let path = dir.path();

            if proc.recursive && path.is_dir() {
                (proc.pre_cb)(&path);
                visit_dirs(&path, &proc)?;
                (proc.post_cb)(&path);
            }
        }
        (proc.post_cb)(&path);
        Ok(())
    }

    pub fn print_cpuset(entry : &Path) {
        let path = entry.to_str().unwrap().to_owned() + "/cpuset";
        match fs::read_to_string(path) {
            Ok(buf) => println!("{:?} {}", entry, buf.trim()),
            Err(_) => println!("{:?}", entry),
        }
    }

    pub fn set_cpuset(entry : &Path, mask : &str) {
        let path = entry.to_str().unwrap().to_owned() + "/cpuset";
        match fs::File::options().write(true).truncate(true).open(path) {
            Ok(mut f) => {
                match f.write_all(mask.as_bytes()) {
                    Ok(_) => println!("{:?} {}", entry, mask),
                    Err(err) => println!("{:?} {}", entry, err),
                }
            },
            Err(err) => println!("{:?} {}", entry, err),
        }
    }

    pub fn create_cpuset(entry : &Path, name : &str) -> bool {
        let path = entry.to_str().unwrap().to_owned() + "/" + name;
        match fs::create_dir(path) {
            Ok(_) => true,
            Err(err) => {
                println!("{:?}: {}", entry, err);
                false
            },
        }
    }

    pub fn destroy_cpuset(entry : &Path) {
        match fs::remove_dir(entry.to_str().unwrap()) {
            Ok(_) => (),
            Err(err) => println!("{:?}: {}", entry, err),
        }
    }
}
