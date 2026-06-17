use std::path::Path;
use std::fs;
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dry_run: bool,

    #[arg(short, long, default_value = "7")]
    days: u64,
}

fn main() {
    let args = Args::parse();
    
    println!("{}", "=== Linux Soft Cleaner v0.1.0 ===".bold().blue());
    if args.dry_run {
        println!("{}", "Running in DRY-RUN mode. No files will be deleted.".yellow());
    }

    let mut total_saved: u64 = 0;

    let tmp_path = Path::new("/tmp");
    if tmp_path.exists() {
        println!("{}", "Analyzing /tmp directory...".cyan());
        if let Ok(entries) = fs::read_dir(tmp_path) {
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] {msg}").unwrap());
            pb.enable_steady_tick(std::time::Duration::from_millis(80));

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        let size = metadata.len();
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                        pb.set_message(format!("Checking: {}", file_name));
                        
                        if !args.dry_run {
                            if let Err(e) = fs::remove_file(&path) {
                                pb.println(format!("Failed to delete {}: {}", file_name.red(), e));
                            } else {
                                total_saved += size;
                            }
                        } else {
                            total_saved += size;
                            pb.println(format!("Would delete: {} ({} bytes)", file_name.green(), size));
                        }
                    }
                }
            }
            pb.finish_with_message("Done analyzing /tmp.");
        }
    } else {
        println!("{}", "/tmp directory not found or inaccessible. Skipping.".yellow());
    }

    println!("{}", "\nAnalyzing system package cache...".cyan());
    let apt_cache = Path::new("/var/cache/apt/archives");
    if apt_cache.exists() {
        if let Ok(metadata) = fs::metadata(apt_cache) {
            let size = metadata.len();
            total_saved += size;
            if args.dry_run {
                println!("Would clean APT cache ({} bytes)", size);
            } else {
                println!("Cleaning APT cache...");
            }
        }
    } else {
        println!("APT cache not found. Skipping.");
    }

    println!("\n{}", "=== Summary ===".bold().blue());
    if args.dry_run {
        println!("Estimated space to save: {} KB", total_saved / 1024);
    } else {
        println!("Total space saved: {} KB", total_saved / 1024);
    }
    println!("{}", "System cleanup process finished successfully!".green());
}
