use std::env::current_dir;

use crate::bindings::Windows::Win32::{
    System::SystemServices::PWSTR,
    UI::{Shell::ShellExecuteW, WindowsAndMessaging::HWND},
};

/** This command kills the current process */
pub(crate) fn taskkill() {
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

//BUG This command does not work
/** This command restarts the Application */
pub(crate) fn restart_app() {
    let mut path = current_dir().unwrap();
    path.push("ace.exe");
    let path = path.to_str().unwrap();
    let result = unsafe { ShellExecuteW(HWND::NULL, "open", path, PWSTR::NULL, PWSTR::NULL, 1) };
    println!("result: {:?}", result);

    taskkill();
}

/** Starts ACE */
pub(crate) fn start_ace() {
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
