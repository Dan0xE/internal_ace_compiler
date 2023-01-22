use crate::bindings::Windows::Win32::{
    System::SystemServices::PWSTR,
    UI::{Shell::ShellExecuteW, WindowsAndMessaging::HWND},
};

/// This command executes a command as admin
pub(crate) fn admin_execute(command: String) {
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
