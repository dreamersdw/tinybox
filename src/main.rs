#[macro_use]
extern crate structopt;
extern crate chrono;

use structopt::StructOpt;

mod loops;
mod sum;

#[derive(StructOpt, Debug)]
#[structopt(name = "tinybox", about = "An collection of cli tools")]
enum Opt {
    #[structopt(name = "sum", about = "calculate the sum of a series of numbers")]
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
        } => {
            loops::loops(interval, count, no_title, cmd);
        }
    }
}
