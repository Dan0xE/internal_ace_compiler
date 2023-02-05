use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    commands::{install::install_command, process::restart_app},
    utils::sigcheck,
};

/// Downloads a file from a url and saves it to a file
pub(crate) fn download_to_file(url: &str, file_name: &str, hash: &str) {
    if fs::metadata(file_name).is_ok() {
        println!("File already exists, skipping download");
        let check = sigcheck::sigcheck(file_name, hash);
        if !check {
            panic!("Hash of downloaded file does not match, aborting");
        }
        return;
    }
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut content = Vec::new();
    resp.copy_to(&mut content).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(&content).unwrap();

    let check = sigcheck::sigcheck(file_name, hash);
    if !check {
        panic!("Hash of downloaded file does not match, aborting");
    }

    println!("Downloaded and checked hash of {}", file_name);
}

/// Downloads the node installer and installs it
pub(crate) fn download_node_installer() {
    println!("Downloading Node.js installer, the app will quit after installation");
    println!("Please restart the app after installation");
    download_to_file(
        "https://nodejs.org/download/release/v16.15.0/node-v16.15.0-x64.msi",
        "node.msi",
        "B1F7290DFBE9B07671B523A33BD07695",
    );

    install_command("node.msi".to_string());

    std::thread::sleep(std::time::Duration::from_secs(60));

    fs::remove_file("node.msi").unwrap();

    println!("Node.js installed");

    restart_app()
}
