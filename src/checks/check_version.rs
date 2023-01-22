use std::process::Command;

use crate::commands::{download::download_node_installer, uninstall::uninstall_node};

/** Checks if the node version is correct (ACE cannot compile under Node v18.x) */
pub(crate) fn check_node_version() -> bool {
    let output = Command::new("node")
        .arg("-v")
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    if output.contains("v16.15.0") {
        println!("Node.js version is correct");
        false
    } else {
        println!("Node.js version is not correct");
        println!("ACE cannot compile under Node version higher than 16.15.0");
        println!("Would you like to uninstall Node.js and install the correct version? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            std::process::exit(0);
        }
        uninstall_node();
        download_node_installer();
        true
    }
}
