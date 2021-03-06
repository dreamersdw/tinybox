#[macro_use]
extern crate structopt;
extern crate chrono;
extern crate fs2;
extern crate hyper;
extern crate notify;

use structopt::StructOpt;

mod bits;
mod gen;
mod http;
mod lock;
mod loops;
mod sum;
mod watch;

#[derive(StructOpt, Debug)]
#[structopt(name = "tinybox", about = "An collection of cli tools")]
enum Opt {
    #[structopt(
        name = "sum",
        about = "calculate the sum of a series of numbers"
    )]
    Sum,

    #[structopt(name = "loop", about = "run a shell command repeatedly")]
    Loop {
        #[structopt(short = "i", long = "interval", default_value = "2.0")]
        interval: f64,

        #[structopt(short = "c", long = "count", default_value = "0")]
        count: usize,

        #[structopt(long = "no-title")]
        no_title: bool,

        #[structopt(name = "cmd", raw(required = "true"))]
        cmd: Vec<String>,
    },

    #[structopt(
        name = "watch",
        about = "run a shell command when any file changed"
    )]
    Watch {
        #[structopt(short = "d", long = "dir", default_value = ".")]
        dir: String,

        #[structopt(short = "w", long = "wait", default_value = "1.0")]
        wait: f64,

        #[structopt(name = "cmd", raw(required = "true"))]
        cmd: Vec<String>,
    },

    #[structopt(
        name = "lock",
        about = "run a shell command when a lock is acquired"
    )]
    Lock {
        #[structopt(
            short = "l",
            long = "lockfile",
            default_value = "/tmp/tinybox-lock.lock"
        )]
        lock_file: String,

        #[structopt(name = "cmd", raw(required = "true"))]
        cmd: Vec<String>,
    },
    #[structopt(
        name = "bits",
        about = "show the bit level layout for given numbers"
    )]
    Bits {
        #[structopt(name = "nums", raw(required = "true"))]
        nums: Vec<u64>,
    },
    #[structopt(name = "http", about = "start a http server")]
    Http {
        #[structopt(name = "port", short = "p", default_value = "3000")]
        port: u16,
    },
    #[structopt(
        name = "gen",
        about = "generate aliases for the tinybox tools"
    )]
    Gen {},
}
fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Sum => sum::sum(),
        Opt::Loop {
            interval,
            count,
            no_title,
            cmd,
        } => loops::loops(interval, count, no_title, &cmd),
        Opt::Watch { dir, wait, cmd } => watch::watch(&dir, wait, &cmd),
        Opt::Lock { lock_file, cmd } => lock::lock(&lock_file, &cmd),
        Opt::Bits { nums } => bits::bits(&nums),
        Opt::Http { port } => http::http(port),
        Opt::Gen {} => gen::gen(),
    }
}
