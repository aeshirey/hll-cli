use csv::ReaderBuilder;
use hyperloglog::HyperLogLog;

#[derive(Debug)]
struct CsvArgs {
    header: bool,
    delimiter: u8,
    columns: Option<String>,
}

fn parse_csv_args(matches: &clap::ArgMatches) -> Result<CsvArgs, ()> {
    let header: bool = matches.is_present("header");
    let delimiter: u8 = matches.value_of("delimiter").unwrap().as_bytes()[0];
    let columns = matches.value_of("columns").map(|v| v.to_string());

    Ok(CsvArgs {
        header,
        columns,
        delimiter,
    })
}

pub fn process_csv(
    inputs: &Vec<&str>,
    error_rate: f64,
    matches: &clap::ArgMatches,
) -> Result<(), ()> {
    let args = parse_csv_args(&matches)?;
    let mut hlls = Vec::new();

    let mut header: Option<Vec<String>> = None;

    for input in inputs {
        let result = process_csv_file(input, &mut hlls, error_rate, &args)?;

        if args.header {
            match (&header, result) {
                (None, Some(hdr)) => {
                    header.replace(hdr);
                }
                _ => {}
            }
        }
    }

    if let Some(cols) = header {
        println!("{}", cols.join(&(args.delimiter as char).to_string()));
    }

    // All files processed - dump results
    println!(
        "{}",
        hlls.iter()
            .map(|hll| (hll.len() as u64).to_string())
            .collect::<Vec<_>>()
            .join(&((args.delimiter as char).to_string()))
    );

    Ok(())
}

fn process_csv_file(
    input: &str,
    hlls: &mut Vec<HyperLogLog<String>>,
    error_rate: f64,
    args: &CsvArgs,
) -> Result<Option<Vec<String>>, ()> {
    let mut reader = ReaderBuilder::new()
        .has_headers(args.header)
        .delimiter(args.delimiter)
        .from_path(input)
        .map_err(|_| ())?;

    let header = match (&args.header, &reader.headers()) {
        (true, Ok(header)) => Some(header.iter().map(|s| s.to_string()).collect()),
        _ => None,
    };

    for line in reader.records() {
        let record = line.map_err(|_| ())?;

        if hlls.len() < record.len() {
            hlls.extend((hlls.len()..record.len()).map(|_| HyperLogLog::new(error_rate)));
        }

        for (value, hll) in record.iter().zip(hlls.iter_mut()) {
            hll.insert(&value.to_string());
        }
    }

    Ok(header)
}
