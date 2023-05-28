use log::info;
use std::{fs, io, path::PathBuf, process::exit};

use clap::Parser;

/// Copies a directory to another location, ignoring files listed in .gitignore files
#[derive(Parser, Debug)]
struct Args {
    /// The path of the directory to copy from
    source: PathBuf,

    /// The path of the directory to copy to
    target: PathBuf,

    /// Be verbose
    #[arg(short, long)]
    verbose: bool,

    /// The name of the file with the list of files to ignore
    #[arg(short, long, default_value = ".gitignore")]
    ignore: String,
}

fn copy(
    current_path: PathBuf,
    target_path: PathBuf,
    args: &Args,
    ignore_list: &mut Vec<PathBuf>,
) -> Result<(), io::Error> {
    info!("Copying {:?} to {:?}", current_path, target_path);

    if let Ok(ignore_file) = fs::read_to_string(current_path.join(&args.ignore)) {
        for line in ignore_file.lines() {
            if line.starts_with('#') {
                continue;
            }

            let line = line.strip_prefix('/').unwrap_or(line);

            let mut ignore_path = current_path.clone();
            ignore_path.push(line);
            info!("Now ignoring {}", ignore_path.display());

            ignore_list.push(ignore_path);
        }
    }

    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if ignore_list.contains(&path) {
            log::info!("Ignoring {:?}", path);
            continue;
        }

        let mut new_target_path = target_path.clone();
        new_target_path.push(path.file_name().unwrap());
        if path.is_dir() {
            fs::create_dir_all(&new_target_path)?;

            copy(path, new_target_path, args, ignore_list)?;
        } else {
            fs::copy(&path, &new_target_path)?;
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    } else {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn"));
    }

    let mut ignore_list: Vec<PathBuf> = Vec::new();
    match copy(
        args.source.clone(),
        args.target.clone(),
        &args,
        &mut ignore_list,
    ) {
        Ok(_) => {}
        Err(e) => {
            log::error!(
                "Error copying {:?} to {:?}: {}",
                args.source,
                args.target,
                e
            );
            exit(1);
        }
    };
}
