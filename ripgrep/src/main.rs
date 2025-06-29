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
    println!("Pattern is: {}", args.pattern.to_lowercase());
    println!("Path is: {:?}\n", args.path);

    let ignores = [".exe", ".psd", ".mp4", ".gitignore", ".tmp"];

    let re = regex::RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .with_context(|| "Could not compile regex")?;

    let walk_buider = WalkBuilder::new(&args.path);

    walk_buider.build_parallel().run(|| {
                let local_re = &re;
                let local_args = &args;

                Box::new(move |result| {
                    let entry = match result {
                        Ok(entry) => entry,
                        Err(err) => {
                            eprint!("Error : {err}");
                            return WalkState::Continue;
                        }
                    };

                    if ignores.iter().any(|i| entry.path().ends_with(i)){
                        return WalkState::Continue
                    }

                    if !entry.file_type().is_some_and(|ft| ft.is_file()){
                        return  WalkState::Continue;
                    }


                    if let Ok(contents) = fs::read_to_string(entry.path()){
                        for (i, line) in contents.lines().enumerate(){
                            let is_match = local_re.is_match(line);

                            if is_match ^ local_args.invert_match {
                                let out = if local_args.invert_match {
                                    format!("{} : {} : {}", entry.path().display().to_string().green(), i+1, line)
                                } else {
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