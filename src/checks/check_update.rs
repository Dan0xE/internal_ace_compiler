use std::process::Command;

use crate::{
    commands::install::install_git, commands::install::install_modules,
    commands::uninstall::uninstall_node_modules, start_ace,
};

use super::check_installation::check_git_if_installed;

/** Checks for updates in the ace repository */
pub(crate) fn check_if_update_is_needed() -> bool {
    if !check_git_if_installed() {
        install_git();
    } else {
        println!("Git already installed");
    }

    let output = Command::new("git")
        .arg("pull")
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    if output.contains("Already up to date") {
        false
    } else {
        uninstall_node_modules();
        install_modules();
        start_ace();
        true
    }
}
