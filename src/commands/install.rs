use std::{env::current_dir, path::Path, thread::sleep};

use crate::{
    bindings::Windows::Win32::{
        System::SystemServices::PWSTR,
        UI::{Shell::ShellExecuteW, WindowsAndMessaging::HWND},
    },
    commands::download::download_to_file,
    commands::process::{restart_app, start_ace},
};

/** This command installs Node **/
pub(crate) fn install_command(package_name: String) {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "runas",
            "C:\\WINDOWS\\system32\\msiexec.exe",
            " /i".to_owned()
                + current_dir().unwrap().to_str().unwrap()
                + "\\"
                + &package_name
                + " /quiet /norestart /log nodeinstall.log",
            PWSTR::NULL,
            //is shown or not 1 = show 0 = hide
            0,
        )
    };
    println!("{:?}", r);
    if r.0 < 32 {
        println!("error: {:?}", r);
    }
}

/** This command checks for an existing git installation and installs git if none is found */
pub(crate) fn install_git() {
    let path = Path::new("C:\\Program Files\\Git\\bin\\git.exe");

    if !path.exists() {
        println!("Git not found, installing...");
        download_to_file("https://github.com/git-for-windows/git/releases/download/v2.38.1.windows.1/Git-2.38.1-64-bit.exe", "git.exe");
        windows::initialize_sta().unwrap();
        let r = unsafe {
            ShellExecuteW(
            HWND::NULL,
            "open",
            "cmd",
            " /c".to_owned() + " " + "git.exe /SILENT /VERYSILENT /NORESTART /NOCANCEL /SP- /CLOSEAPPLICATIONS /RESTARTAPPLICATIONS /NORESTARTAPPLICATIONS /SUPPRESSMSGBOXES /DIR=C:\\Program Files\\Git",
            PWSTR::NULL,
            //is shown or not 1 = show 0 = hide
            0,
        )
        };
        if r.0 < 32 {
            println!("error: {:?}", r);
        }
        restart_app();
    } else {
        println!("Git already installed");
    }
}

/** Installs node modules */
pub(crate) fn install_modules() {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "open",
            "cmd",
            " /c".to_owned() + " " + "npm install --force",
            PWSTR::NULL,
            //is shown or not 1 = show 0 = hide
            0,
        )
    };
    if r.0 < 32 {
        println!("error: {:?}", r);
    }

    let mut counter = 0;
    while !Path::new("node_modules").exists() {
        std::thread::sleep(std::time::Duration::from_secs(5));
        counter += 1;
        if counter > 10 {
            println!("Could not create folder, exiting");
            let mut line = String::new();
            println!("Press any key to exit");
            let stdin = std::io::stdin();
            stdin.read_line(&mut line).unwrap();
            std::process::exit(0);
        }
    }

    sleep(std::time::Duration::from_secs(220));

    println!("Node modules installed");

    start_ace();
}
