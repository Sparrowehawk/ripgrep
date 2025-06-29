use std::fs::{self};
use anyhow::{Result, Context};
use ignore::{WalkBuilder, WalkState};
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
    // Troubleshooting commands
    // println!("Pattern is: {}", args.pattern.to_lowercase());
    // println!("Path is: {:?}\n", args.path);

    // So that we don't read anything bad
    let ignores = [".exe", ".psd", ".mp4", ".gitignore", ".tmp"];

    // Creates a regex of the pattern and a multi-thread builder
    let re = regex::RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .with_context(|| "Could not compile regex")?;

    let walk_buider = WalkBuilder::new(&args.path);

    walk_buider.build_parallel().run(|| {
                // Local vars, tbh they are redundent but make readability better
                let local_re = &re;
                let local_args = &args;

                // Per thread for each line in each file

                Box::new(move |result| {
                    let entry = match result {
                        Ok(entry) => entry,
                        Err(err) => {
                            eprint!("Error : {err}");
                            return WalkState::Continue;
                        }
                    };
                    
                    // Removes files from earlier include
                    if ignores.iter().any(|i| entry.path().ends_with(i)){
                        return WalkState::Continue
                    }

                    // Makes sure its a file not a dir
                    if !entry.file_type().is_some_and(|ft| ft.is_file()){
                        return  WalkState::Continue;
                    }


                    if let Ok(contents) = fs::read_to_string(entry.path()){
                        // For every line, create an int with it and search per
                        for (i, line) in contents.lines().enumerate(){
                            // Cross refrence with the is_match from the struct to determine what to do
                            let is_match = local_re.is_match(line);

                            // Xor means that it chooses one, not both. Good error handling imo
                            if is_match ^ local_args.invert_match {
                                let out = if local_args.invert_match {
                                    format!("{} : {} : {}", entry.path().display().to_string().green(), i+1, line)
                                } else {
                                    // Split the result into pre and post indexed words
                                    let mat = local_re.find(line).unwrap();
                                    format!("{} : {} : {} {} {}", 
                                            entry.path().display().to_string().green(),
                                            i + 1,
                                            &line[.. mat.start()],
                                            &line[mat.start() .. mat.end()].blue().bold(),
                                            &line[mat.end()..]
                                        )
                                };

                                println!("{out}")
                            }
                        }
                    }

                    WalkState::Continue
                })
            });

    Ok(())
}