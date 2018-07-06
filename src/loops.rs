use super::chrono;

use std::process::exit;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn loops(interval: f64, count: usize, no_title: bool, cmd: Vec<String>) {
    let shell = cmd.join(" ");

    let mut total = 1;
    loop {
        if !no_title {
            let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("{} every {}s: {}", now, interval, shell);
        } else {
            println!();
        }

        let mut command = Command::new("sh");
        match command.arg("-c").arg(&shell).spawn() {
            Ok(mut handle) => {
                handle.wait().unwrap();
            }
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }

        if count != 0 && total >= count {
            break;
        }

        sleep(Duration::from_millis((interval * 1000.0) as u64));
        total += 1;
    }
}
