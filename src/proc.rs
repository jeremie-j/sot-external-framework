use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{DWORD, HINSTANCE__, HMODULE, LPCVOID, LPVOID};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Module32Next, Process32Next, MODULEENTRY32, PROCESSENTRY32,
    TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
};
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};

struct MemoryReader {
    exe: String,
    base_adress: u32,
    handle: HANDLE,
    process_id: DWORD,
}

impl MemoryReader {
    fn new(exe: &str) -> MemoryReader {
        let process_id = unsafe { get_proc_id(exe) };
        let handle = Self::get_process_handle(process_id);
        let base_adress = unsafe { get_module_base_address(process_id, exe) } as u32;
        let exe = String::from(exe);

        MemoryReader {
            exe,
            base_adress,
            handle,
            process_id,
        }
    }

    fn get_process_handle(proc_id: DWORD) -> HANDLE {
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, proc_id) }
    }
}

pub fn get_module_handle(name: *const u8) -> HMODULE {
    unsafe { GetModuleHandleA(name as _) }
}

pub unsafe fn get_proc_id(exe_name: &str) -> DWORD {
    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if snapshot == INVALID_HANDLE_VALUE {
        panic!("Failed to create snapshot");
    }
    let mut entry = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; 260],
    };
    let mut proc_id: DWORD = 0;
    while Process32Next(snapshot, &mut entry) != 0 {
        if exe_name
            == std::ffi::CStr::from_ptr(entry.szExeFile.as_ptr())
                .to_str()
                .unwrap()
        {
            proc_id = entry.th32ProcessID;
            break;
        }
    }
    return proc_id;
}

pub fn get_process_adress(module: HMODULE, proc_name: *const u8) -> *mut c_void {
    unsafe { GetProcAddress(module, proc_name as _) as _ }
}

pub fn find_dma_addy(handle: HANDLE, ptr: u32, ptrs: &[u32]) -> u32 {
    unsafe {
        let mut addr = ptr;
        let mut buffer = 0;
        for offset in ptrs {
            println!("offset: {:#X}, address: {:#X}", offset, addr);
            addr += offset;
            println!("addr: {:#X}", addr);
            let success = ReadProcessMemory(
                handle,
                addr as *mut _,
                &mut buffer as *mut u32 as *mut c_void,
                4,
                0 as *mut _,
            );
            if success == 0 {
                panic!("Failed to read memory");
            };
            addr = buffer;
        }
        addr
    }
}

pub fn read_bytes(handle: HANDLE, adress: u32, byte: usize) -> u32 {
    unsafe {
        let mut buffer: u32 = 0;
        let success = ReadProcessMemory(
            handle,
            adress as LPCVOID,
            &mut buffer as *mut u32 as LPVOID,
            byte as SIZE_T,
            null_mut(),
        );
        if success == 0 {
            panic!("Failed to read memory");
        };
        buffer
    }
}
