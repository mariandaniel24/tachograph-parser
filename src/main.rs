use anyhow::{Context, Result};
use clap::{value_parser, Arg, Command};
use flexi_logger::Logger;
use std::fs;
use std::path::PathBuf;
use tachograph_parser::{process_file_json, FileType};

fn main() -> Result<()> {
    let matches = Command::new("Tachograph Parser")
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("input_type")
                .value_parser(["vu", "card"])
                .required(true)
                .help("Type of input file (VU or Card)"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_parser(value_parser!(PathBuf))
                .required(true)
                .help("Input file path"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_parser(value_parser!(PathBuf))
                .required(true)
                .help("Output file path"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::Count)
                .help("Enable verbose logging"),
        )
        .get_matches();

    let file_type = match matches.get_one::<String>("input_type").unwrap().as_str() {
        "vu" => FileType::VU,
        "card" => FileType::Card,
        _ => unreachable!(),
    };

    let input = matches.get_one::<PathBuf>("input").unwrap();
    let output = matches.get_one::<PathBuf>("output").unwrap();

    // Set up logging if verbose flag is used
    if matches.get_count("verbose") > 0 {
        std::env::set_var("RUST_LOG", "trace");
        Logger::try_with_env()?
            .start()
            .context("Failed to start logger")?;
    }

    let json_output = process_file_json(file_type, input.to_str().unwrap())
        .context("Failed to process input file")?;

    fs::write(output, json_output).context("Failed to write output file")?;

    println!("Processing complete. Output written to: {:?}", output);

    Ok(())
}
