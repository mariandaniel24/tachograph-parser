use anyhow::Result;
use std::env;
use tachograph_parser::process_driver_card_file_json;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    flexi_logger::Logger::try_with_env()?.start()?;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_driver_card_file>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let card_data = process_driver_card_file_json(path)?;
    std::fs::write(format!("{path}.json"), card_data)?;

    Ok(())
}
