extern crate fs2;

use fs2::FileExt;
use std::fs::OpenOptions;
use std::process::{exit, Command};

pub fn lock(lock_file: String, cmd: &[String]) {
    let f = match OpenOptions::new().create(true).write(true).open(&lock_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open lock file error {}", e);
            exit(1);
        }
    };

    let mut first_time = true;
    loop {
        match f.try_lock_exclusive() {
            Ok(_) => {
                if !first_time {
                    eprintln!("ok");
                }
                break;
            }
            Err(_) => {
                if first_time {
                    eprint!("Blocking: wait for lock ... ");
                    first_time = false;
                }
            }
        }
    }

    let shell = cmd.join(" ");
    let mut command = Command::new("sh");
    match command.arg("-c").arg(&shell).spawn() {
        Ok(mut handle) => {
            handle.wait().unwrap();
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
