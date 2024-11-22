use std::{fmt::Debug, fs, path::PathBuf};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "photo-import")]
struct Opt {
    #[structopt(parse(from_os_str))]
    source: PathBuf,

    #[structopt(parse(from_os_str))]
    destination: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Opt {
        source,
        destination,
    } = Opt::from_args();

    if !source.is_dir() {
        return Err(anyhow!("Source must be a directory"));
    }

    if !destination.exists() {
        println!("Creating destination directory: {:?}", destination);
        std::fs::create_dir_all(&destination)?;
    }

    println!(
        "Copying files from {:?} to {:?}. Continue? (Y/n)",
        source, destination
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let answer = input
        .trim()
        .chars()
        .into_iter()
        .nth(0)
        .unwrap_or_default()
        .to_ascii_lowercase();

    if answer != 'y' {
        return Ok(());
    }

    let source_files = std::fs::read_dir(&source)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("Found {} files to copy", source_files.len());

    for source_file in source_files {
        let file_name = source_file
            .file_name()
            .ok_or_else(|| anyhow!("Invalid file name"))?;

        let metadata = fs::metadata(&source_file)?;
        let created: DateTime<Utc> = metadata.created()?.into();

        let mut destination_dir = destination.join(created.format("%b-%e-%Y").to_string());

        if let Some(extension) = source_file.extension() {
            if extension.to_ascii_lowercase() == "dng" {
                destination_dir.push("raw");
            }
        }

        if !destination_dir.exists() {
            println!("Creating destination directory: {:?}", destination_dir);
            std::fs::create_dir_all(&destination_dir)?;
        }

        let destination_file = destination_dir.join(file_name);

        println!("Copying {:?} to {:?}", source_file, destination_file);
        fs::copy(&source_file, &destination_file)?;
    }

    Ok(())
}
