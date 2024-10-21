use anyhow::Result;
use std::env;
use std::time::Instant;
use tachograph_parser::parse_card_from_bytes;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    flexi_logger::Logger::try_with_env()?.start()?;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_driver_card_file>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let bytes = std::fs::read(path)?;
    let start = Instant::now();
    let card_data = parse_card_from_bytes(&bytes)?;
    let duration = start.elapsed();
    println!(
        "Parsing took {} nanos ({:.2} ms)",
        duration.as_nanos(),
        duration.as_secs_f64() * 1000.0
    );
    std::fs::write(format!("{path}.json"), serde_json::to_string(&card_data)?)?;

    Ok(())
}
