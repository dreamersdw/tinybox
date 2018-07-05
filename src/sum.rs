use std::io::{stdin, BufRead};
use std::process::exit;

pub(crate) fn sum() {
    let mut total: f64 = 0.0;

    let input = stdin();
    for (idx, line) in input.lock().lines().enumerate() {
        let r = line.map(|text| text.parse::<f64>());
        match r {
            Ok(Ok(v)) => total += v,
            Ok(Err(e)) => {
                eprintln!("{}, at line {}", e, idx + 1);
                exit(1);
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    println!("{}", total);
}
