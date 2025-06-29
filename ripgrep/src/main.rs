use std::fs::File;
use std::io::prelude::*;

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
    println!("Path is: {:?}", args.path);

    let mut file = File::open(args.path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    println!("{contents}");

    Ok(())
}