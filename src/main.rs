use clap::{App, Arg};

mod csv;
mod text;

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
            Arg::with_name("format")
            .short("f")
            .long("format")
            .default_value("text")
                .help("Input file format. Omit for line-based text.")
                .long_help(
                    r#"Input file format. Use 'csv' to split the input text into columns, counting
each column independently. You may also use "#,
                )
                .required(false),
        )
        .arg(
                Arg::with_name("header")
                    .short("h")
                    .long("header")
                    .multiple(false)
                    .takes_value(false)
                    .help("Indicates the input has a header. This flag is implicitly set. Use -H to disable header."),
        )
        .arg(
                Arg::with_name("no-header")
                    .short("H")
                    .long("no-header")
                    .multiple(false)
                    .takes_value(false)
                    .help("Indicates the input does not have a header"),
        )
        .arg(
                Arg::with_name("delimiter")
                    .short("d")
                    .long("delimiter")
                    .default_value(",")
                    .multiple(false)
                    .help("The column delimiter. Default is ','")
        )
        .arg(
                Arg::with_name("columns")
                    .short("c")
                    .long("columns")
                    .multiple(false)
                    .help("A list of column names, numbers, or number ranges")
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

    let inputs = match matches.values_of("input") {
        Some(values) => values,
        None => {
            println!("No input files specified");
            return Err(());
        }
    };

    let inputs: Vec<&str> = inputs.collect();

    match matches.value_of("format").unwrap() {
        "csv" => csv::process_csv(&inputs, error_rate, &matches)?,
        "json" => todo!("json formatted input"),
        "parquet" => todo!("parquet formatted input"),
        _ => text::process_text(&inputs, error_rate)?
    }

    Ok(())
}
