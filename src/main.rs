use checks::check_installation::check_node_installation;
use checks::check_modules::check_node_modules;
use checks::check_update::check_if_update_is_needed;

use crate::commands::process::start_ace;

mod bindings {
    windows::include_bindings!();
}

mod commands {
    pub(crate) mod download;
    pub(crate) mod install;
    pub(crate) mod process;
    pub(crate) mod uninstall;
}

mod methods {
    pub(crate) mod execution;
}

mod checks {
    pub(crate) mod check_installation;
    pub(crate) mod check_modules;
    pub(crate) mod check_update;
    pub(crate) mod check_version;
}

mod utils {
    pub(crate) mod sigcheck;
}

fn main() {
    if !check_node_installation() && !check_node_modules() && !check_if_update_is_needed() {
        start_ace();
    }
}
