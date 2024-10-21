use anyhow::Result;
use std::env;
use tachograph_parser::parse_vu_from_file_to_json;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    flexi_logger::Logger::try_with_env()?.start()?;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_vu_file>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let card_data = parse_vu_from_file_to_json(path)?;

    std::fs::write(format!("{path}.json"), card_data)?;
    // println!("Vehicle Unit Data: {:?}", card_data);

    Ok(())
}
