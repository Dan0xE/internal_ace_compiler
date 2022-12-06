use std::thread::sleep;
use std::{env::current_dir, fs, fs::File, io::Write, path::Path, process::Command};

mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::System::SystemServices::PWSTR;
use bindings::Windows::Win32::UI::Shell::ShellExecuteW;
use bindings::Windows::Win32::UI::WindowsAndMessaging::HWND;

fn restart_app() {
    let mut path = current_dir().unwrap();
    path.push("ace.exe");
    let path = path.to_str().unwrap();
    let result = unsafe { ShellExecuteW(HWND::NULL, "open", path, PWSTR::NULL, PWSTR::NULL, 1) };
    println!("result: {:?}", result);

    taskkill();
}

fn taskkill() {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "runas",
            "C:\\Windows\\System32\\taskkill.exe",
            "/PID ".to_owned() + std::process::id().to_string().as_str() + " /T " + "/F",
            // PWSTR::from(command),
            PWSTR::NULL,
            //is shown or not 1 = show 0 = hide
            0,
        )
    };
    if r.0 < 32 {
        println!("error: {:?}", r);
    }
}

fn admin_execute(command: String) {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "runas",
            "C:\\Windows\\System32\\cmd.exe",
            // PWSTR::from(command),
            "/c".to_string() + " " + &command,
            PWSTR::NULL,
            //is shown or not 1 = show 0 = hide
            0,
        )
    };
    if r.0 < 32 {
        println!("error: {:?}", r);
    }
}

fn install_command(package_name: String) {
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

fn uninstall_node() {
    admin_execute("wmic product where name=\"Node.js\" call uninstall /nointeractive".to_string());
    std::thread::sleep(std::time::Duration::from_secs(20));
    println!("Node.js uninstalled");
}

fn download_to_file(url: &str, file_name: &str) {
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut content = Vec::new();
    resp.copy_to(&mut content).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(&content).unwrap();

    println!("Downloaded {}", file_name);
}

fn install_git() {
    let path = Path::new("C:\\Program Files\\Git\\bin\\git.exe");

    if !path.exists() {
        println!("Git not found, installing...");
        download_to_file("https://github.com/git-for-windows/git/releases/download/v2.38.1.windows.1/Git-2.38.1-64-bit.exe", "git.exe");
        install_command("git.exe /VERYSILENT /NORESTART /SP- ".to_string());
        restart_app();
    } else {
        println!("Git already installed");
    }
}

fn check_git_if_installed() -> bool {
    let output = Command::new("C:\\Program Files\\Git\\bin\\git.exe").output();

    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn remove_node_modules() {
    fs::remove_dir_all("node_modules").unwrap();
}

fn check_if_update_is_needed() -> bool {
    if !check_git_if_installed() {
        install_git();
    }

    let output = Command::new("git")
        .arg("pull")
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    if output.contains("Already up to date") {
        false
    } else {
        remove_node_modules();
        install_modules();
        start_app();
        true
    }
}

fn download_node_installer() {
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

fn install_modules() {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "Run ",
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
            println!("Node modules not installed, trying again");
            install_modules();
        }
    }

    sleep(std::time::Duration::from_secs(220));

    println!("Node modules installed");

    start_app();
}

fn start_app() {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "open",
            "cmd",
            // PWSTR::from(command),
            " /c".to_owned() + " " + "npm start",
            PWSTR::NULL,
            //is shown or not 1 = show 0 = hide
            0,
        )
    };
    if r.0 < 32 {
        println!("error: {:?}", r);
    }

    println!("App started");

    //press any key to exit
    println!("Press any key to exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::process::exit(0);
}

fn check_node_modules() -> bool {
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

fn check_node_version() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        let output = String::from_utf8_lossy(&output.stdout);
        if output.contains("v16.15.0") {
            false
        } else {
            uninstall_node();
            download_node_installer();
            true
        }
    } else {
        download_node_installer();
        true
    }
}

fn main() {
    if !check_node_version() && !check_node_modules() && !check_if_update_is_needed() {
        start_app();
    }
}
