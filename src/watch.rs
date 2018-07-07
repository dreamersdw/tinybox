use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::process::exit;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::Instant;

pub fn watch(dir: &str, wait: f64, cmd: Vec<String>) {
    let (tx, rx) = channel();
    let delay = Duration::from_millis((wait * 1000.0) as u64);
    let mut watcher: RecommendedWatcher = match Watcher::new(tx, delay) {
        Ok(w) => w,
        Err(e) => {
            println!("{}", e);
            exit(-1);
        }
    };

    if let Err(e) = watcher.watch(&dir, RecursiveMode::Recursive) {
        println!("{}", e);
        exit(1);
    }
    let shell = cmd.join(" ");
    let mut last_run: Option<Instant> = None;
    loop {
        match rx.recv() {
            Ok(_) => {
                if let Some(t) = last_run {
                    if t.elapsed() < Duration::from_millis(500) {
                        continue;
                    }
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

                last_run = Some(Instant::now());
            }

            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
