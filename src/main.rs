use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(author, version, about = "A fast, safe, and minimal disk cleanup tool for Linux servers", long_about = None)]
struct Args {
    #[arg(short, long, help = "Run in dry-run mode (preview changes, no files will be deleted)")]
    dry_run: bool,

    #[arg(short, long, default_value = "7", help = "Threshold in days for temporary files and logs")]
    days: u64,

    #[arg(long, help = "Also clean Docker system caches (containers, images, volumes)")]
    docker: bool,

    #[arg(long, help = "Also clean global NPM cache")]
    npm: bool,
}

fn is_older_than_days(metadata: &fs::Metadata, days: u64) -> bool {
    if let Ok(modified) = metadata.modified() {
        if let Ok(elapsed) = modified.elapsed() {
            return elapsed.as_secs() > days * 24 * 3600;
        }
    }
    false
}

fn clean_directory_files<F>(dir_path: &Path, args: &Args, filter: F, description: &str) -> u64
where
    F: Fn(&Path, &fs::Metadata) -> bool,
{
    let mut saved_bytes = 0;
    if !dir_path.exists() {
        return 0;
    }

    println!("{} {}...", "Analyzing".cyan(), description);
    
    // Simple spinner for progress
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut files_to_clean = Vec::new();

    // Helper to recursively collect files
    fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    collect_files(&path, files);
                }
            }
        }
    }

    collect_files(dir_path, &mut files_to_clean);

    for path in files_to_clean {
        if let Ok(metadata) = fs::metadata(&path) {
            let size = metadata.len();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            pb.set_message(format!("Checking: {}", file_name));

            if filter(&path, &metadata) {
                if args.dry_run {
                    pb.println(format!("{} {} ({:.2} MB)", "Would delete:".green(), path.display(), size as f64 / 1_048_576.0));
                    saved_bytes += size;
                } else {
                    match fs::remove_file(&path) {
                        Ok(_) => {
                            pb.println(format!("{} {}", "Deleted:".red(), path.display()));
                            saved_bytes += size;
                        }
                        Err(e) => {
                            pb.println(format!("{} failed to delete {}: {}", "Error:".red(), path.display(), e));
                        }
                    }
                }
            }
        }
    }

    pb.finish_with_message(format!("Finished cleaning {}.", description));
    saved_bytes
}

fn main() {
    let args = Args::parse();

    println!("{}", "=== Linux Soft Cleaner v0.1.0 ===".bold().blue());
    if args.dry_run {
        println!("{}", "Running in DRY-RUN mode. No changes will be written.".yellow());
    }
    println!("File/Log age threshold: {} days\n", args.days.to_string().green());

    let mut total_saved: u64 = 0;

    // 1. Clean /tmp
    total_saved += clean_directory_files(
        Path::new("/tmp"),
        &args,
        |_, meta| is_older_than_days(meta, args.days),
        "temporary directory (/tmp)",
    );

    // 2. Clean Package Manager Caches
    // APT Cache (Debian/Ubuntu)
    total_saved += clean_directory_files(
        Path::new("/var/cache/apt/archives"),
        &args,
        |path, _| path.extension().map_or(false, |ext| ext == "deb"),
        "APT package cache",
    );

    // DNF/YUM Cache (RHEL/CentOS/Fedora)
    total_saved += clean_directory_files(
        Path::new("/var/cache/dnf"),
        &args,
        |path, _| path.extension().map_or(false, |ext| ext == "rpm"),
        "DNF package cache",
    );
    total_saved += clean_directory_files(
        Path::new("/var/cache/yum"),
        &args,
        |path, _| path.extension().map_or(false, |ext| ext == "rpm"),
        "YUM package cache",
    );

    // Pacman Cache (Arch Linux)
    total_saved += clean_directory_files(
        Path::new("/var/cache/pacman/pkg"),
        &args,
        |path, _| path.extension().map_or(false, |ext| ext == "pkg.tar.zst" || ext == "pkg.tar.xz"),
        "Pacman package cache",
    );

    // 3. System Log Rotation files
    total_saved += clean_directory_files(
        Path::new("/var/log"),
        &args,
        |path, meta| {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let is_rotated = filename.contains(".log.") || filename.ends_with(".gz") || filename.ends_with(".1") || filename.ends_with(".old");
            is_rotated && is_older_than_days(meta, args.days)
        },
        "rotated system logs (/var/log)",
    );

    // 4. Optional: Docker Prune
    if args.docker {
        println!("\n{}", "Cleaning Docker system caches...".cyan());
        if args.dry_run {
            println!("{} Would execute 'docker system prune -a --volumes -f'", "Dry-Run:".yellow());
        } else {
            let output = Command::new("docker")
                .args(["system", "prune", "-a", "--volumes", "-f"])
                .output();
            match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    println!("{}", stdout);
                    // Parse reclaimed space from docker output if possible
                    if let Some(line) = stdout.lines().last() {
                        if line.contains("Total reclaimed space:") {
                            println!("{}", line.green());
                        }
                    }
                }
                Err(e) => {
                    println!("{} Could not execute docker command: {}", "Error:".red(), e);
                }
            }
        }
    }

    // 5. Optional: NPM Global Cache
    if args.npm {
        let home_dir = std::env::var("HOME").ok().map(PathBuf::from);
        if let Some(mut npm_cache_path) = home_dir {
            npm_cache_path.push(".npm/_cacache");
            total_saved += clean_directory_files(
                &npm_cache_path,
                &args,
                |_, _| true, // clean all cache files
                "global NPM cache (~/.npm/_cacache)",
            );
        }
    }

    println!("\n{}", "=== Summary ===".bold().blue());
    let saved_mb = total_saved as f64 / 1_048_576.0;
    if args.dry_run {
        println!("Estimated space to save: {:.2} MB", saved_mb);
    } else {
        println!("Total space saved: {:.2} MB", saved_mb);
    }
    println!("{}", "System cleanup completed successfully!".green());
}
