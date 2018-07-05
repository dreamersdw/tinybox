#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod sum;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "sum")]
    Sum,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Sum => sum::sum(),
    }
}
