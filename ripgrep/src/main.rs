use std::fs::{self};
use anyhow::{Result, Context};
use ignore::Walk;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String, 
    path: std::path::PathBuf,

    // Flag time
    /// Make the search case-insensitive
    #[arg(short = 'i', long)]
    ignore_case: bool,

    /// Invert the search to find lines that DON'T match
    #[arg(short = 'v', long)]
    invert_match: bool,
}


fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Pattern is: {}", args.pattern.to_lowercase());
    println!("Path is: {:?}\n", args.path);

    let ignores = [".exe", ".psd", ".mp4", ".gitignore", ".tmp"];

    let re = regex::RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .with_context(|| "Could not compile regex")?;


    for entry in Walk::new(args.path)
                .filter_map(Result::ok)
                .filter(|e| {
                    !ignores.iter().any(|i| e.path().ends_with(i))
                })
                .filter(|e| e.file_type().expect("REASON").is_file()){
                    let path = fs::read_to_string(entry.path())
                                   .with_context(|| format!("Failed to read from {}", entry.path().display()))?; 

                    for (i, line) in path.lines().enumerate() {
                        let is_match = re.is_match(line);

                        if is_match ^ args.invert_match {
                            if args.invert_match {
                                println!("{}: {}: {}", entry.path().display().to_string().green(), i + 1, line);
                            } else {
                                let mat = re.find(line).unwrap();
                                let start = mat.start();
                                let end = mat.end();

                                
                                print!("{}: {}: {}", entry.path().display().to_string().green(), i + 1, &line[..start]);
                                print!("{}", &line[start..end].blue().bold());
                                println!("{}", &line[end..]);
                            }
                        }
                    }
                };

    Ok(())
}