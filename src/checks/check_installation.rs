use crate::commands::download::download_node_installer;
use std::process::Command;

use crate::checks::check_version::check_node_version;

/** Checks if git is installed */
pub(crate) fn check_git_if_installed() -> bool {
    let output = Command::new("C:\\Program Files\\Git\\bin\\git.exe").output();

    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}

/** Checks if node is installed */
pub(crate) fn check_node_installation() -> bool {
    let output = Command::new("node").arg("--version").output();

    match output {
        Ok(_) => {
            println!("Node.js is installed");
            return check_node_version();
        }

        Err(_) => {
            println!("Node.js not found, installing...");
            download_node_installer();
            true
        }
    }
}
