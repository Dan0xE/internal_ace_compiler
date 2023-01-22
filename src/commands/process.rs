use std::env::current_dir;

use crate::bindings::Windows::Win32::{
    System::SystemServices::PWSTR,
    UI::{Shell::ShellExecuteW, WindowsAndMessaging::HWND},
};

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

pub(crate) fn restart_app() {
    let mut path = current_dir().unwrap();
    path.push("ace.exe");
    let path = path.to_str().unwrap();
    let result = unsafe { ShellExecuteW(HWND::NULL, "open", path, PWSTR::NULL, PWSTR::NULL, 1) };
    println!("result: {:?}", result);

    taskkill();
}
