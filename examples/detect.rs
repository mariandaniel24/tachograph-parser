use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tachograph_parser::detector::{self, TachoFileType};

fn main() -> Result<()> {
    let data_dir = Path::new("data/ddd");
    let mut file_types: HashMap<TachoFileType, Vec<String>> = HashMap::new();

    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_path = path.to_str().unwrap().to_string();
            match detector::detect_from_file(&file_path) {
                Ok(file_type) => {
                    file_types.entry(file_type).or_default().push(file_path);
                }
                Err(e) => {
                    println!("Error detecting file type for {}: {}", file_path, e);
                }
            }
        }
    }

    for (file_type, files) in file_types.iter() {
        println!("Type: {:?}", file_type);
        for file in files {
            println!("  - {}", file);
        }
        println!();
    }

    Ok(())
}
