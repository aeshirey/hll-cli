use hyperloglog::HyperLogLog;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_text(inputs: &Vec<&str>, error_rate: f64) -> Result<(), ()> {
    let mut hll = HyperLogLog::<String>::new(error_rate);

    for input in inputs {
        match input {
            &"-" => process_stdin(&mut hll),
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
    const CAPACITY: usize = 1022;

    let in_fh = File::open(filename).map_err(|_e| ())?;
    let in_buf = BufReader::with_capacity(CAPACITY, in_fh);

    for line in in_buf.lines() {
        let line = line.unwrap();
        hll.insert(&line);
    }

    Ok(())
}
