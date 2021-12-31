use std::{ffi::c_void};
use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::{WriteProcessMemory, ReadProcessMemory},
        Memory::{VirtualProtectEx, PAGE_READWRITE},
    },
};

#[allow(dead_code)]
pub unsafe fn WriteMemory(
    hProc: HANDLE,
    lpAddress: *const c_void,
    lpBuf: *const c_void,
    nSize: usize,
    lpNumOfBytesWritten: *mut usize,
)
{
    let mut new = PAGE_READWRITE;
    let mut old = PAGE_READWRITE;
    VirtualProtectEx(hProc, lpAddress, nSize, new, &mut old);
    WriteProcessMemory(hProc, lpAddress, lpBuf, nSize, lpNumOfBytesWritten);
    VirtualProtectEx(hProc, lpAddress, nSize, old, &mut new);
}

#[allow(dead_code)]
pub unsafe fn ReadMemory(
    hProc: HANDLE,
    lpAddress: *const c_void,
    lpBuf: *mut c_void,
    nSize: usize,
    lpNumOfBytesWritten: *mut usize,
)
{
    let mut new = PAGE_READWRITE;
    let mut old = PAGE_READWRITE;
    VirtualProtectEx(hProc, lpAddress, nSize, new, &mut old);
    ReadProcessMemory(hProc, lpAddress, lpBuf, nSize, lpNumOfBytesWritten);
    VirtualProtectEx(hProc, lpAddress, nSize, old, &mut new);
}

/*
pub unsafe fn WriteMem<T>(lpAddress: *mut T, lpBuf: *const T, count: usize)
{
    let mut old = PAGE_READWRITE;
    let mut new = PAGE_READWRITE;
    let nSize = count * size_of::<T>();
    VirtualProtect(lpAddress as *const c_void, nSize, new, &mut old);
    ReadProcessMemory(hprocess, lpbaseaddress, lpbuffer, nsize, lpnumberofbytesread)
    VirtualProtect(lpAddress as *const c_void, nSize, old, &mut new);
}

pub unsafe fn ReadMem<T>(lpAddress: *const T, lpBuf: *mut T, count: usize)
{
    let mut old = PAGE_READONLY;
    let mut new = PAGE_READONLY;
    let nSize = count * size_of::<T>();
    VirtualProtect(lpAddress as *const c_void, nSize, new, &mut old);
    copy_nonoverlapping::<T>(lpAddress, lpBuf, count);
    VirtualProtect(lpAddress as *const c_void, nSize, old, &mut new);
}
 */
