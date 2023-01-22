use std::{
    fs::{self, File},
    io::Write,
};

use crate::commands::{install::install_command, process::restart_app};

/// Downloads a file from a url and saves it to a file
pub(crate) fn download_to_file(url: &str, file_name: &str) {
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut content = Vec::new();
    resp.copy_to(&mut content).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(&content).unwrap();

    println!("Downloaded {}", file_name);
}

/// Downloads the node installer and installs it (sigcheck missing!)
pub(crate) fn download_node_installer() {
    download_to_file(
        "https://nodejs.org/download/release/v16.15.0/node-v16.15.0-x64.msi",
        "node.msi",
    );

    install_command("node.msi".to_string());

    std::thread::sleep(std::time::Duration::from_secs(60));

    fs::remove_file("node.msi").unwrap();

    println!("Node.js installed");

    restart_app()
}
