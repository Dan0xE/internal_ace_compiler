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
    println!("Installing {}", package_name);
    println!("Would you like to install quietly? (No installer Window) (y/n)");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    if line.trim() == "y" {
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
                0,
            )
        };
        println!("Successfully installed {}", package_name);
        if r.0 < 32 {
            eprintln!("Error installing package: {:?}", r);
        }
    } else {
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
                    + " /log nodeinstall.log",
                PWSTR::NULL,
                1,
            )
        };
        if r.0 < 32 {
            eprintln!("Error installing package: {:?}", r);
        }
    }
}

/** This command checks for an existing git installation and installs git if none is found */
pub(crate) fn install_git() {
    let path = Path::new("C:\\Program Files\\Git\\bin\\git.exe");

    if !path.exists() {
        println!("Git not found");
        println!("Would you like to install git? (y/n)");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() != "y" {
            std::process::exit(0);
        }
        download_to_file("https://github.com/git-for-windows/git/releases/download/v2.38.1.windows.1/Git-2.38.1-64-bit.exe", "git.exe", "8E3F77F7CA96AC3402E8D28B43409F38");

        println!("Would you like to install git quietly? (No installer Window) (y/n)");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "y" {
            windows::initialize_sta().unwrap();
            let r = unsafe {
                ShellExecuteW(
				HWND::NULL,
				"open",
				"cmd",
				" /c".to_owned() + " " + "git.exe /SILENT /VERYSILENT /NORESTART /NOCANCEL /SP- /CLOSEAPPLICATIONS /RESTARTAPPLICATIONS /NORESTARTAPPLICATIONS /SUPPRESSMSGBOXES /DIR=C:\\Program Files\\Git",
				PWSTR::NULL,
				0,
			)
            };
            if r.0 < 32 {
                eprintln!("error - failed to execute command. hresult: {:?}", r);
            }
            restart_app();
        } else {
            windows::initialize_sta().unwrap();
            let r = unsafe {
                ShellExecuteW(
                    HWND::NULL,
                    "open",
                    "cmd",
                    " /c".to_owned() + " " + "git.exe /DIR=C:\\Program Files\\Git",
                    PWSTR::NULL,
                    1,
                )
            };
            if r.0 < 32 {
                eprintln!("error: {:?}", r);
            }
            restart_app();
        }
    } else {
        println!("Git already installed");
    }
}

/** Installs node modules */
pub(crate) fn install_modules() {
    if !Path::new("package.json").exists() {
        eprintln!("No package.json file found, exiting");
        let mut line = String::new();
        println!("Press any key to exit");
        let stdin = std::io::stdin();
        stdin.read_line(&mut line).unwrap();
        std::process::exit(0);
    }
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "open",
            "cmd",
            " /c".to_owned() + " " + "npm install --force",
            PWSTR::NULL,
            0,
        )
    };
    if r.0 < 32 {
        eprintln!(
            "Error occured while trying to install node modules: {:?}",
            r
        );
    }

    let mut counter = 0;
    while !Path::new("node_modules").exists() {
        std::thread::sleep(std::time::Duration::from_secs(5));
        counter += 1;
        if counter > 2 {
            eprintln!("Could not create folder, exiting");
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
