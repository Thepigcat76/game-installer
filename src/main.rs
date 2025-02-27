use std::{fs::File, io::{self, BufWriter}, path::Path, process::Command};

use reqwest::blocking::Client;
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

fn main() -> anyhow::Result<()> {
    download_python("https://www.python.org/ftp/python/3.11.4/python-3.11.4-amd64.exe", "python_installer.exe")?;
    install_python("python_installer.exe")
}

fn install_python<P: AsRef<Path>>(installer_path: P) -> anyhow::Result<()> {
    let status = Command::new(installer_path.as_ref())
        .arg("/quiet")  // Silent installation (Windows)
        .arg("PrependPath=1")  // Add Python to PATH
        .status()?;

    if status.success() {
        println!("Python installed successfully!");
    } else {
        eprintln!("Python installation failed.");
    }

    Ok(())
}

fn download_python<P: AsRef<Path>>(url: &str, output_path: P) -> anyhow::Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send()?.error_for_status()?;

    let mut file = File::create(&output_path)?;
    io::copy(&mut response, &mut file)?;

    println!("Downloaded Python installer to {:?}", output_path.as_ref());
    Ok(())
}

fn download_release<P: AsRef<Path>>(url: &str, path: P) -> anyhow::Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send()?.error_for_status()?;

    let mut file = File::create(path)?;
    io::copy(&mut response, &mut file)?;
    Ok(())
}

fn zip_directory<P: AsRef<Path>>(src_dir: P, output_zip: P) -> anyhow::Result<()> {
    let zip_file = File::create(output_zip)?;
    let writer = BufWriter::new(zip_file);
    let mut zip = ZipWriter::new(writer);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated) // Use Deflate compression
        .unix_permissions(0o755);

    for entry in WalkDir::new(&src_dir) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(&src_dir)?.to_str().unwrap();

        if path.is_dir() {
            zip.add_directory(name.to_string(), options)?;
        } else {
            let mut file = File::open(path)?;
            zip.start_file(name.to_string(), options)?;
            std::io::copy(&mut file, &mut zip)?;
        }
    }

    zip.finish()?; // Finalize the ZIP archive
    Ok(())
}