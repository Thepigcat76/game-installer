use std::{
    env, fs::{self, File}, io, path::Path
};

use anyhow::Context;
use reqwest::blocking::Client;
use zip::ZipArchive;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    download_release(
        &format!("https://github.com/Thepigcat76/reference/releases/download/v{}/game.zip", args.get(1).map(String::as_str).unwrap_or("1.0.0")),
        "game.zip",
    )?;
    let dir = home::home_dir().context("Failed to find home directory")?;
    let path = dir.join("game");
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    let to = path.join("game.zip");

    empty_dir(&path)?;

    fs::rename("game.zip", &to)?;

    let file = File::open(&to)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = path.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    fs::remove_file(to)?;

    Ok(())
}

fn empty_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn download_release<P: AsRef<Path>>(url: &str, path: P) -> anyhow::Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send()?.error_for_status()?;

    let mut file = File::create(path)?;
    io::copy(&mut response, &mut file)?;
    Ok(())
}
