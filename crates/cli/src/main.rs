use anyhow::{Context, Result};
use clap::{value_parser, Arg, Command};
use flexi_logger::Logger;
use std::fs;
use std::path::PathBuf;
use tachograph_parser::{
    detector::{self, TachoFileType},
    parse_card_from_file_to_json, parse_vu_from_file_to_json,
};

fn main() -> Result<()> {
    let matches = Command::new("Tachograph Parser")
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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

    let input = matches
        .get_one::<PathBuf>("input")
        .unwrap()
        .to_str()
        .unwrap();
    let output = matches.get_one::<PathBuf>("output").unwrap();

    let detected_file_type = detector::detect_from_file(input)?;

    // Set up logging if verbose flag is used
    if matches.get_count("verbose") > 0 {
        std::env::set_var("RUST_LOG", "trace");
        Logger::try_with_env()?
            .start()
            .context("Failed to start logger")?;
    }

    let json_output = match detected_file_type {
        TachoFileType::VehicleUnitGen1
        | TachoFileType::VehicleUnitGen2
        | TachoFileType::VehicleUnitGen2V2 => {
            parse_vu_from_file_to_json(input).context("Failed to process input file")?
        }
        TachoFileType::DriverCardGen1
        | TachoFileType::DriverCardGen2
        | TachoFileType::DriverCardGen2V2 => {
            parse_card_from_file_to_json(input).context("Failed to process input file")?
        }
    };

    fs::write(output, json_output).context("Failed to write output file")?;

    println!(
        "Processing of {} complete with file type: {:?}. Output written to: {}",
        input,
        detected_file_type,
        output.to_str().unwrap()
    );

    Ok(())
}
