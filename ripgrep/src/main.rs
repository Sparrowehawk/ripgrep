use std::fs::{self};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String, 
    path: std::path::PathBuf,
}


fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    println!("Pattern is: {}", args.pattern);
    println!("Path is: {:?}\n", args.path);

    let contents = fs::read_to_string(&args.path)?;

    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!("{line}");
        }
    }

    Ok(())
}