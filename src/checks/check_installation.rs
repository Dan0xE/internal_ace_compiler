use std::process::Command;

pub(crate) fn check_git_if_installed() -> bool {
    let output = Command::new("C:\\Program Files\\Git\\bin\\git.exe").output();

    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}
