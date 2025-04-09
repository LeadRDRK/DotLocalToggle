#![windows_subsystem = "windows"]

use windows::{core::{w, HSTRING}, Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK}};

fn run() -> Result<(), registry::Error> {
    let regkey = registry::Hive::LocalMachine.open(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options",
        registry::Security::Read | registry::Security::SetValue
    )?;

    let current = regkey.value("DevOverrideEnable")
        .ok()
        .map(|v| match v {
            registry::Data::U32(v) => v,
            _ => 0
        })
        .unwrap_or(0);
    let new = if current == 0 { 1 } else { 0 };

    regkey.set_value("DevOverrideEnable", &registry::Data::U32(new))?;

    unsafe {
        MessageBoxW(
            None,
            &HSTRING::from(format!(
                "DotLocal DLL redirection is now {}. Restart your computer to apply the changes.",
                if new == 1 { "enabled" } else { "disabled" }
            )),
            w!("DotLocal Toggle"),
            MB_ICONINFORMATION | MB_OK
        );
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        unsafe { MessageBoxW(None, &HSTRING::from(e.to_string()), w!("Error"), MB_ICONERROR | MB_OK); }
    }
}
