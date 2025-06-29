use std::fs::{self};
use anyhow::{Result, Context};
use regex::Regex;

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

    let re = Regex::new(&args.pattern)
        .with_context(|| "Could not compile regex")?;

    let contents = fs::read_to_string(&args.path)
                      .with_context(|| format!("Failed to read from {}", args.path.display()))?;

    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line){
            println!("{}. {line}", i+1);
        }
    }

    Ok(())
}