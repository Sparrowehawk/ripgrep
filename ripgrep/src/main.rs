use std::fs::{self};
use anyhow::{Result, Context};
use regex::Regex;
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String, 
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Pattern is: {}", args.pattern.to_lowercase());
    println!("Path is: {:?}\n", args.path);

    let ignores = [".exe", ".psd", ".mp4", ".gitignore", ".tmp"];

    let re = Regex::new(&args.pattern)
        .with_context(|| "Could not compile regex")?;


    for entry in WalkDir::new(args.path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| {
                    !ignores.iter().any(|i| e.path().ends_with(i))
                })
                .filter(|e| e.file_type().is_file()){
                    let path = fs::read_to_string(entry.path())
                                   .with_context(|| format!("Failed to read from {}", entry.path().display()))?; 

                    for (i, line) in path.lines().enumerate() {
                        if re.is_match(line){
                            println!("Found in : {}", entry.path().display());
                            println!("{}. {line}", i+1);
                        }
                    }
                };

    Ok(())
}