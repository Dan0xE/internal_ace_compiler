use std::fs;

use crate::methods::execution::admin_execute;

/// Uninstalls Node.js
pub(crate) fn uninstall_node() {
    admin_execute("wmic product where name=\"Node.js\" call uninstall /nointeractive".to_string());
    std::thread::sleep(std::time::Duration::from_secs(20));
    println!("Node.js uninstalled");
}

/// Removes the node_modules folder
pub(crate) fn uninstall_node_modules() {
    fs::remove_dir_all("node_modules").unwrap();
}
