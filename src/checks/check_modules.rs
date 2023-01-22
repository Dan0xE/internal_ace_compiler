use std::path::Path;

use crate::commands::install::install_modules;

/** Installs node modules if they are not installed yet */
pub(crate) fn check_node_modules() -> bool {
    let path = Path::new("node_modules");
    match path.metadata() {
        Ok(meta) => {
            if meta.is_dir() {
                println!("node_modules already installed");
                false
            } else {
                println!("node_modules is not a directory, installing...");
                install_modules();
                true
            }
        }
        Err(_) => {
            println!("node_modules not found, installing...");
            install_modules();
            true
        }
    }
}
