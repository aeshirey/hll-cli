use clap::{App, Arg};
use hyperloglog::HyperLogLog;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), ()> {
    let matches = App::new("HyperLogLog CLI")
        .version("0.1")
        .author("Adam Shirey <adam@shirey.ch>")
        .about("Efficiently approximates distinct count of input values.")
        .arg(
            Arg::with_name("error-rate")
                .short("e")
                .long("error-rate")
                .value_name("ERROR_RATE")
                .default_value("0.05")
                .help("The acceptable error rate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .default_value("-")
                .help("Specifies the input source")
                .takes_value(true),
        )
        .get_matches();

    let error_rate = matches
        .value_of("error-rate")
        .unwrap()
        .parse::<f64>()
        .map_err(|e| {
            println!("Failed to parse the error rate argument: {}", e);
        })?;

    if error_rate <= 0.0 || error_rate >= 1.0 {
        println!("Error rate must be between 0 and 1, inclusive.");
        return Err(());
    }

    let input = matches.value_of("input").unwrap();

    let hll = match input {
        "-" => hll_from_stdin(error_rate),
        filename => hll_from_file(filename, error_rate)?,
    };

    println!("{}", hll.len().round() as u64);

    Ok(())
}

fn hll_from_stdin(error_rate: f64) -> HyperLogLog<String> {
    let mut hll = HyperLogLog::new(error_rate);
    let eols: &[_] = &['\r', '\n'];
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) if n > 0 => {
                let trimmed = input.trim_end_matches(eols).to_string();
                hll.insert(&trimmed)
            }
            _ => break,
        }
    }

    hll
}

fn hll_from_file(filename: &str, error_rate: f64) -> Result<HyperLogLog<String>, ()> {
    const CAPACITY: usize = 1024;
    let mut hll = HyperLogLog::new(error_rate);

    let in_fh = File::open(filename).map_err(|_e| ())?;
    let in_buf = BufReader::with_capacity(CAPACITY, in_fh);

    for line in in_buf.lines() {
        let line = line.unwrap();
        hll.insert(&line);
    }

    Ok(hll)
}
