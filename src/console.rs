use windows_sys::Win32::System::Console::{
    AllocConsole,
    FreeConsole,
};

pub fn enable() {
    unsafe {
        AllocConsole();
    }
}

pub fn disable() {
    unsafe {
        FreeConsole();
    }
}