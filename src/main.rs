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
                .default_value("0.01")
                .help("The approximate allowable error rate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .help("Specifies the input source. If omitted or '-', stdin is used.")
                .default_value("-")
                .multiple(true),
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

    //let input = matches.value_of("input").unwrap();
    let inputs = match matches.values_of("input") {
        Some(values) => values,
        None => {
            println!("No input files specified");
            return Err(());
        }
    };

    let mut hll = HyperLogLog::<String>::new(error_rate);

    for input in inputs {
        match input {
            "-" => process_stdin(&mut hll),
            filename => process_file(&mut hll, filename)?,
        }
    }

    println!("{}", hll.len().round() as u64);

    Ok(())
}

fn process_stdin(hll: &mut HyperLogLog<String>) {
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
}

fn process_file(hll: &mut HyperLogLog<String>, filename: &str) -> Result<(), ()> {
    const CAPACITY: usize = 1024;

    let in_fh = File::open(filename).map_err(|_e| ())?;
    let in_buf = BufReader::with_capacity(CAPACITY, in_fh);

    for line in in_buf.lines() {
        let line = line.unwrap();
        hll.insert(&line);
    }

    Ok(())
}
