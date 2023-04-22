use winapi::{
    ctypes::c_void,
    shared::minwindef::HMODULE,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
};

pub fn get_module_handle(name: *const u8) -> HMODULE {
    unsafe { return GetModuleHandleA(name as _) }
}

pub fn get_proc_address(module: HMODULE, name: *const u8) -> *const c_void {
    unsafe { return GetProcAddress(module, name as _) as _ }
}
