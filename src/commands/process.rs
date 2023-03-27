use crate::bindings::Windows::Win32::{
    System::SystemServices::PWSTR,
    UI::{Shell::ShellExecuteW, WindowsAndMessaging::HWND},
};
use std::process::{exit, Command};

/** This command restarts the Application */
pub(crate) fn restart_app() -> ! {
    let mut command = Command::new(std::env::args().next().unwrap());
    command.args(std::env::args().skip(1));
    let status = command.spawn();
    if let Err(e) = status {
        eprintln!("Failed to restart process: {}", e);
        exit(1);
    } else {
        exit(0);
    }
}

/** Starts ACEV2 */
pub(crate) fn start_ace() {
    windows::initialize_sta().unwrap();
    let r = unsafe {
        ShellExecuteW(
            HWND::NULL,
            "open",
            "cmd",
            " /c".to_owned() + " " + "npm start",
            PWSTR::NULL,
            1,
        )
    };
    if r.0 < 32 {
        println!("error: {:?}", r);
    }

    println!("App started");

    println!("Press any key to exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::process::exit(0);
}
