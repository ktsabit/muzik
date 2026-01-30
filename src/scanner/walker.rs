use crate::scanner::metadata::{get_audio_info, print_audio_info};
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

const SUPPORTED_AUDIO_EXTS: &[&str] = &["mp3", "flac", "ogg", "m4a", "wav", "aac", "opus"];

pub fn walkdir(root_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        let lit_ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        if !SUPPORTED_AUDIO_EXTS.contains(&lit_ext) {
            continue;
        }

        match verify_audio_file(path) {
            Ok(actual_ext) => {
                println!(
                    "File: {} | Literal: {} | Actual: {}",
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown"),
                    lit_ext,
                    actual_ext
                );

                match get_audio_info(path) {
                    Ok(metadata) => print_audio_info(&metadata),
                    Err(e) => eprintln!("Failed to read {:?}: {}", path, e),
                }
            }
            Err(e) => {
                eprintln!("Failed to verify {:?}: {}", path, e);
            }
        }
    }
    Ok(())
}

fn verify_audio_file(path: &std::path::Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 16];
    file.read(&mut buffer)?;

    let actual_ext = infer::get(&buffer)
        .map(|k| k.extension().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Ok(actual_ext)
}
