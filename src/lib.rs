#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::ptr::null_mut;
use windows::Win32::Foundation::HINSTANCE;
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

/*
lazy_static!
{
    static ref NRHANDLE : Mutex<HANDLE> = Mutex::new(HANDLE(0));
    static ref NRMAP : Mutex<usize> = Mutex::new(0);
    static ref NRIMAGEBASE : Mutex<usize> = Mutex::new(0);
    static ref NRNRRes : Mutex<NR::NRRes> = Mutex::new(NR::NRRes{..Default::default()});
    static ref NRint : Mutex<usize> = Mutex::new(0);
}
*/

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
/*
#[no_mangle]
pub extern "system" fn DllMain(hInst : HINSTANCE, fdwReason : usize, pvReserved: *const c_void )
{
    let DPA = DLL_PROCESS_ATTACH as usize;
    let DPD = DLL_PROCESS_DETACH as usize;

    let hHandle : *mut HANDLE = 0 as *mut HANDLE;
    let map : *mut *mut c_void = 0 as *mut *mut c_void;

    match fdwReason
    {
        DPA  => Init(hHandle, map),
        DPD => unsafe {Detach(*map,*hHandle)},
    }
}
*/

fn Init()
{
    //panic!("hi!");
    let mut a_vec = Box::new(Vec::<String>::new());
    let mut b_vec = Box::new(Vec::<String>::new());
    NR::File::ReadTxtToVec("a.txt", &mut a_vec);
    NR::File::ReadTxtToVec("b.txt", &mut b_vec);
    unsafe {
        NR::Base::DebugOutput(format!("hi!"));
        let hHandle = OpenProcess(PROCESS_ALL_ACCESS, false, GetCurrentProcessId());
        let ExEdit = GetModuleHandleA("exedit.auf").0;
        let pIDH: *const IMAGE_DOS_HEADER = ExEdit as *const IMAGE_DOS_HEADER;

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

        //panic!("test");

        //let mut i = 0u32;
        //let mut pIRDE = (pIRD as usize + size_of::<IMAGE_RESOURCE_DIRECTORY>())
        //    as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
        //NR::DebugOutput(format!("{:x}",pIRDE as u32));
        //let mut nrresvec = Vec::<NR::Base::Res>::with_capacity(1024);
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
        /*
        while i < (IdEntryNum as u32 + NameEntryNum as u32)
        {
            NR::DebugOutput(format!("Entry Num : {:x}",IdEntryNum as u32 + NameEntryNum as u32));
            pIRDE = pIRDE.offset(1);
            i += 1;
        }
         */
        println!("Resource : {:?}",Resource);
        NR::Res::showRes(&Resource);
        //NR::DebugOutput(format!("{:?}",nrresvec));
        VirtualProtect(resRVA as *const c_void, resIDD.Size as usize, old, &mut new);
        /*
        let SecNum = (*pINH).FileHeader.NumberOfSections;
        //let SecHead = (ImageBase + (*pIDH).e_lfanew as u32 + size_of::<IMAGE_FILE_HEADER>() as u32 +(*pINH).FileHeader.SizeOfOptionalHeader as u32) as *const IMAGE_SECTION_HEADER;
        let SecHead = pINH as u32 + size_of::<IMAGE_FILE_HEADER>() as u32 + (*pINH).FileHeader.SizeOfOptionalHeader as u32 + 4;
        let mut SecHead = SecHead as *mut IMAGE_SECTION_HEADER;



        let mut adr = Vec::<usize>::with_capacity(1024);
        let mut resadr = Vec::<usize>::with_capacity(1024);
        let mut i = 0u16;




        let mut SecHeaders = Vec::<IMAGE_SECTION_HEADER>::with_capacity(1024);

        while i < SecNum
        {
            SecHeaders.push(*SecHead);
            i += 1;
            SecHead = NR::NextSecHead(SecHead);
        }


        //DebugOutput(format!("{:x}",SecHead as u32));

        for SecHead in SecHeaders.clone()
        {
            if SecHead.Characteristics.0 == 0x40000040
            {
                let a = ImageBase + SecHead.VirtualAddress
            }
        }

        //.rsrcから該当する文字列のアドレス収納
        for SecHead in SecHeaders.clone()
        {
            let flag = SecHead.Characteristics.0;
            DebugOutput(format!("{:x}",flag));
            if flag == 0x40000040 || flag == 0xC0000040
            {
                let mut pExEdit = (ImageBase+SecHead.VirtualAddress) as *const u8;
                let mut i = 1usize;
                let mut j = 0usize;
                let mut l  = 0usize;
                DebugOutputstr("w");
                DebugOutput(format!("{:x}",(*a_vec).clone().len()));
                for a in (*a_vec).clone()
                {
                    let hex = match hex::decode(a.clone())
                    {
                        Ok(o) => o,
                        Err(err) => {
                            DebugOutput(format!("err : {:?}", err));
                            DebugOutput(format!("i : {:x}", i));
                            panic!();
                        },
                    };
                    while j < SecHead.SizeOfRawData as usize
                    {
                        if (*pExEdit) == hex[l]
                        {
                            if l == hex.len() -1
                            {
                                resadr.push(pExEdit as usize - l);
                                l = 0;
                                break;
                            }
                            l += 1;
                        }
                        else { l = 0; }
                        pExEdit = (pExEdit as usize + 1) as *const u8;
                        j+= 1;
                    }
                    i += 1;
                }
            }
        }
        resadr.shrink_to_fit();

        DebugOutput(format!("resadr {:x}",resadr.len()));
        //let mut u32ptr :*const usize;

        //上に該当するアドレスを全部探す
        let mut i = 0u32;
        for SecHead in SecHeaders.clone()
        {
            let flag = SecHead.Characteristics.0;
            if flag != 0x40000040 && flag != 0xC0000040
            {
                let mut ptr = (ImageBase+ SecHead.VirtualAddress) as *const u8;
                while i < SecHead.SizeOfRawData
                {
                    //DebugOutput(format!("ptr : {:x}",ptr as usize));
                    for j in resadr.clone()
                    {
                        let a = j.swap_bytes().to_be_bytes();
                        let mut b = 0usize;
                        let mut c= ptr.clone();
                        loop
                        {
                            if b == a.len()
                            {
                                adr.push(ptr as usize);
                                break;
                            }
                            if *c == a[b]
                            {
                                c = (c as usize + 1) as *const u8;
                                b += 1;
                            }
                            else {break;}
                        }
                    }
                    ptr = (ptr as usize + 1) as *const u8;
                    i += 1;
                }
            }
        }
        adr.shrink_to_fit();

        for addr in adr.clone()
        {
            DebugOutput(format!("{:x}",addr));
        }

        let SMsize = 0x10000usize;

        //Shared Memory - win32
        let hSharedMemory = CreateFileMappingA(
            INVALID_HANDLE_VALUE ,
            null_mut(),
            PAGE_READWRITE,
            0,
            SMsize as u32,
            "NRSH_JP2OL"
        );
        *(NRHANDLE.lock().unwrap()) = hSharedMemory;
        if (hSharedMemory.0 as *mut u8).is_null()
        {
            DebugOutputstr("Failed File Mapping");
            panic!("Failed File Mapping");
        }
        else
        {
            DebugOutputstr("Success");

            let map = MapViewOfFile(
                hSharedMemory,
                 FILE_MAP_ALL_ACCESS,
                 0,
                 0,
                 SMsize
            );

            *(NRMAP.lock().unwrap())= map as usize;

            let mut map = map as *mut u8;
            let mut advec = Vec::<usize>::with_capacity(1024);

            for str in (*b_vec).clone()
            {
                let hexwitherr = hex::decode(str);
                let hex = match hexwitherr{
                    Ok(a) => {
                        a
                    },
                    Err(a) => {
                        DebugOutputstr("err");
                        panic!("{:?}",a);
                    },
                };
                advec.push(map as usize);
                for u in hex
                {
                    *map = u;
                    map = (map as usize + size_of::<u8>()) as *mut u8;
                }
                map = (map as usize + size_of::<u8>()) as *mut u8;
            }
            let mut j = 0usize;
            let mut k = 0usize;
            DebugOutput(format!("{:x}",resadr.len()));

            let mut a = true;
            while j < resadr.len()
            {
                while k < adr.len()
                {
                    let aadr = adr[k] as *mut usize;

                    if *aadr == resadr[j]
                    {
                        let RA = advec[j];
                        let pRA = std::ptr::addr_of!(RA) as *const c_void;
                        let mut foo = 0usize;
                        DebugOutput(format!("{:x}",aadr as usize));
                        DebugOutput(format!("{:x}",*aadr as usize));
                        NR::WriteMemory(hHandle, aadr as *const c_void, pRA, size_of::<usize>(), &mut foo as *mut usize);
                        DebugOutput(format!("{:x}",RA as usize));
                        DebugOutput(format!("{:x}",pRA as usize));
                        DebugOutput(format!("{:x}",foo));
                        adr.swap_remove(k);
                        a = false;
                    }
                    if a
                    {
                        k += 1;
                    }
                    else { a = true; }

                }
                k = 0;
                j += 1;
            }

            /*
            for i in SecHeaders
            {
                if i.Name == CString::new(".rsrc").unwrap() as c_char[]
                {
                    let a = (ImageBase + i.VirtualAddress) as *const IMAGE_RESOURCE_DIRECTORY;
                    let id_entry = (*a).NumberOfIdEntries;
                    let name_entry = (*a).NumberOfNamedEntries;
                    let b = CString::new(".rsrc").unwrap();
                    while
                }
            }
             */
        }
        DebugOutputstr("finished");
        */
    }
}

/*
fn Detach()
{
    unsafe
    {
        UnmapViewOfFile(*(NRMAP.lock().unwrap()) as *mut c_void);
        CloseHandle(*(NRHANDLE.lock().unwrap()));
    }
}
 */
