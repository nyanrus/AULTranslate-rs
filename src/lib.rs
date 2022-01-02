#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::Write;
use std::ptr::null_mut;
use windows::Win32::Foundation::{HINSTANCE, PSTR};
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_READWRITE};
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, IMAGE_DOS_HEADER, IMAGE_RESOURCE_DIRECTORY,
};
use windows::Win32::System::Threading::{GetCurrentProcessId, OpenProcess, PROCESS_ALL_ACCESS};

//use bytes::{Buf, BufMut, BytesMut};
use cty::*;

mod NR;

#[no_mangle]
pub extern "stdcall" fn DllMain(hinst: HINSTANCE, reason: u32, reserved: *mut c_void) -> i32
{
    match reason {
        DLL_PROCESS_DETACH => { /* Detach(); */ }
        DLL_PROCESS_ATTACH => {
            Init();
        }
        _ => {}
    };
    return 1;
}

fn Init()
{
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();

    //panic!("hi!");
    let mut a_vec = Box::new(Vec::<String>::new());
    let mut b_vec = Box::new(Vec::<String>::new());
    NR::File::ReadTxtToVec("a.txt", &mut a_vec);
    NR::File::ReadTxtToVec("b.txt", &mut b_vec);
    unsafe {
        NR::Base::DebugOutput(format!("hi!"));
        let hHandle = OpenProcess(PROCESS_ALL_ACCESS, false, GetCurrentProcessId());
        let mut _ExEditstr = String::from("exedit.auf").as_mut_ptr();
        let ExEdit = GetModuleHandleA(PSTR(_ExEditstr));
        let pIDH: *const IMAGE_DOS_HEADER = ExEdit as *const IMAGE_DOS_HEADER;

        //println!("{:x}",ExEdit);
        if pIDH == null_mut() {
            NR::Base::DebugOutput(format!("pIDH is null"));
            panic!();
        }

        let pINH = (pIDH as i32 + (*pIDH).e_lfanew) as *const IMAGE_NT_HEADERS32;
        let IDD = (*pINH).OptionalHeader.DataDirectory;
        let ImageBase = (*pINH).OptionalHeader.ImageBase;

        let resIDD = IDD[2];
        let relocIDD = IDD[5];

        let resRVA = resIDD.VirtualAddress;
        let relocRVA = relocIDD.VirtualAddress;

        let mut old = PAGE_READWRITE;
        let mut new = PAGE_READWRITE;
        VirtualProtect(resRVA as *const c_void, resIDD.Size as usize, new, &mut old);
        println!("{:x}", resIDD.Size);
        let pIRD = (ImageBase + resRVA) as *const IMAGE_RESOURCE_DIRECTORY;
        //panic!("test");
        let IdEntryNum = (*pIRD).NumberOfIdEntries;
        let NameEntryNum = (*pIRD).NumberOfNamedEntries;

        let mut a = NR::Base::NRTrait {
            hHandle : hHandle,
            ImageBase: ImageBase as usize,
            ResRVA: resRVA as usize,
            ..Default::default()
        };
        let mut IRDEarrvec = Vec::<[NR::Base::ImgResDirEnt; 3]>::new();
        let mut Resource = NR::Base::Resource::new();
        NR::Res::ReadImgResDir(pIRD, &mut a, &mut IRDEarrvec, &mut Resource);
        println!("finish");
        //println!("Resource : {:?}",Resource);
        NR::Res::showRes(&Resource);
        //NR::DebugOutput(format!("{:?}",nrresvec));
        VirtualProtect(resRVA as *const c_void, resIDD.Size as usize, old, &mut new);
        let tmp = (ImageBase + relocRVA) as usize;
        let mut vecptr = NR::Reloc::ReadRelocAll(tmp);
        vecptr.sort();

        let mut file = std::fs::File::create("reloc.txt").unwrap();
        let mut file2 = std::fs::File::create("reloc2.txt").unwrap();
        let mut usizevec = Vec::<usize>::new();
        for i in vecptr
        {
            file.write(format!("{:x}\n",i).as_bytes()).unwrap();
            usizevec.push(*((i + ImageBase as usize) as *const usize));
        }
        usizevec.sort();
        for i in usizevec
        {
            file2.write(format!("{:x}\n", i).as_bytes()).unwrap();
        }
    }
}