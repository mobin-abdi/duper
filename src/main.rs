mod duplicate;
mod file;
mod scanner;

use owo_colors::OwoColorize;
use std::env;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
        }

        "--version" | "-v" => {
            println!("duper {}", env!("CARGO_PKG_VERSION"));
        }

        path => {
            let path = Path::new(path);

            let files = scanner::scan_directory(path);
            let candidates = duplicate::find_candidates(files);
            let duplicates = duplicate::find_duplicates(candidates);

            println!("{}", "Duplicates found:".green().bold());

            for group in &duplicates {
                println!("---");

                for file in group {
                    println!(
                        "{} | {:.4} MB",
                        file.path.display().blue(),
                        (file.size as f64 / 1024.0 / 1024.0).yellow()
                    );
                }
            }

            print!("{}", "\nDelete these files? [Y/N]: ".red());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().eq_ignore_ascii_case("y") {
                for group in &duplicates {
                    for file in group.iter().skip(1) {
                        match std::fs::remove_file(&file.path) {
                            Ok(_) => {
                                println!("{} {}", "Deleted:".green(), file.path.display().red());
                            }
                            Err(error) => {
                                eprintln!(
                                    "Failed to delete {}: {}",
                                    file.path.display(),
                                    error.red()
                                );
                            }
                        }
                    }
                }
            } else {
                println!("{}", "No files were deleted.".yellow());
            }
        }
    }
}

fn print_help() {
    println!("{}", "duper - Duplicate File Finder".cyan().bold());
    println!();
    println!("USAGE:");
    println!("    duper <PATH>");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Show this help message");
    println!("    -v, --version    Show version information");
}
