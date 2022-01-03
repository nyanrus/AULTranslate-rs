use core::mem::size_of;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_SECTION_HEADER, IMAGE_DATA_DIRECTORY};
use windows::Win32::System::SystemServices::{IMAGE_RESOURCE_DIRECTORY, IMAGE_RESOURCE_DATA_ENTRY};
use windows::Win32::System::SystemServices::{
    IMAGE_RESOURCE_DIRECTORY_ENTRY, IMAGE_RESOURCE_DIR_STRING_U,
};

use super::*;

#[deprecated]
#[allow(dead_code)]
pub fn NextSecHead(SecHead: *mut IMAGE_SECTION_HEADER) -> *mut IMAGE_SECTION_HEADER
{
    return (SecHead as u32 + size_of::<IMAGE_SECTION_HEADER>() as u32)
        as *mut IMAGE_SECTION_HEADER;
}

#[allow(dead_code)]
unsafe fn NextIRDir(u: usize) -> usize
{
    return u + size_of::<IMAGE_RESOURCE_DIRECTORY>();
}

pub unsafe fn ReadImgResDir(
    pImgResDir: *const IMAGE_RESOURCE_DIRECTORY,
    pNRTrait: &mut Base::NRTrait,
    pVec: &mut Vec<[Base::ImgResDirEnt; 3]>,
    refRes : &mut Base::Resource,
)
{
    //let mut Res = Base::Resource::new();
    //println!("readimgresdir");
    let num_of_entries = (*pImgResDir).NumberOfIdEntries as usize + (*pImgResDir).NumberOfNamedEntries as usize;
    let mut i = 0usize;
    let mut pImgResDirEnt = (pImgResDir as usize + size_of::<IMAGE_RESOURCE_DIRECTORY>()) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
    while i < num_of_entries
    {
        //println!("inwhile");
        //println!("pimgresdir : {:x}",pImgResDir as usize);
        //println!("num of entries : {:x}",num_of_entries);
        refRes.Nextptr.push(Base::Resource_1 { IsData: false, Res: Box::new(Base::Resource::new()), ..Default::default()});
        let len = refRes.Nextptr.len();
        ReadImgResDirEnt(pImgResDirEnt, pNRTrait, pVec, refRes.Nextptr[len-1].Res.as_mut());
        pImgResDirEnt = pImgResDirEnt.offset(1);
        i += 1;
    }
}

pub unsafe fn NameId(
    pImgResDirEnt: *const IMAGE_RESOURCE_DIRECTORY_ENTRY,
    refNRTrait : &mut Base::NRTrait,
) -> Base::Resource_0
{
    let bitfield = (*pImgResDirEnt).Anonymous1.Anonymous._bitfield;
    // if utf-16 string
    if bitfield & 0x80000000 == 0x80000000
    {
        //println!("1");
        let mut adr = (bitfield & 0x7FFFFFFF) as usize;
        adr += (*refNRTrait).ImageBase + (*refNRTrait).ResRVA;
        let adr = adr as *const IMAGE_RESOURCE_DIR_STRING_U;
        let mut i = 0u16;
        let mut stringadr = (adr as usize + size_of::<u16>()) as *const u16;
        let mut u16vec = Vec::<u16>::new();
        while i < (*adr).Length
        {
            u16vec.push(*stringadr);
            i += 1;
            stringadr = stringadr.offset(1);
        }
        return Base::Resource_0{ IsName: true, Id: Default::default(), Name: String::from_utf16_lossy(&u16vec)};
    }
    else
    {
        //println!("2");
        return Base::Resource_0{IsName: false, Id: (bitfield & 0xFFFF) as u16, Name: Default::default() };
    }
}

pub unsafe fn ReadImgResDirEnt(
    pImgResDirEnt: *const IMAGE_RESOURCE_DIRECTORY_ENTRY,
    refNRTrait: &mut Base::NRTrait,
    refVec: &mut Vec<[Base::ImgResDirEnt; 3]>,
    refRes : &mut Base::Resource,
)
{
    //let mut Res = Base::Resource::new();
    refRes.NameId = NameId(pImgResDirEnt, refNRTrait);
    let bitfield = (*pImgResDirEnt).Anonymous2.Anonymous._bitfield;
    //if next is dir
    let adr = (bitfield & 0x7FFFFFFF) as usize + (*refNRTrait).ImageBase + (*refNRTrait).ResRVA;
    if bitfield & 0x80000000 != 0
    {
        //println!("3");
        ReadImgResDir(adr as *const IMAGE_RESOURCE_DIRECTORY, refNRTrait, refVec, refRes);
    }
    else
    {
        //println!("4");
        let a = adr as *const IMAGE_RESOURCE_DATA_ENTRY;
        let mut ptr = ((*a).OffsetToData as usize + (*refNRTrait).ImageBase) as *const u8;
        let mut i = 0u32;
        let mut vec = Vec::<u8>::new();
        while i < (*a).Size
        {
            vec.push(*ptr);
            ptr = ptr.offset(1);
            i += 1;
        }
        //println!("4-fin");
        refRes.Nextptr.push(Base::Resource_1 { IsData: true, Data: vec.clone(), CodePage: (*a).CodePage, ..Default::default()});
    }
}

pub unsafe fn showRes(refRes : &Base::Resource)
{
    let mut i = 0usize;
    if refRes.NameId.IsName
    {
        println!("Name : {}",refRes.NameId.Name);
    }
    else
    {
        println!("Id : {:x}",refRes.NameId.Id);
    }
    
    while i < refRes.Nextptr.len()
    {
        println!("i : {:x}", i);
        if refRes.Nextptr[i].IsData
        {
            println!("CodePage : {}",refRes.Nextptr[i].CodePage);
            println!("Data : {:?}",refRes.Nextptr[i].Data);
        }
        else
        {
            showRes(refRes.Nextptr[i].Res.as_ref());
        }
        i += 1;
    }
}

pub unsafe fn Res2rsrc(refRes : &Base::Resource) //-> Vec<u8>
{
    
}

pub unsafe fn MakeImgResDir(NamedEntries : u16, IdEntries : u16) -> IMAGE_RESOURCE_DIRECTORY
{
    return IMAGE_RESOURCE_DIRECTORY{
        Characteristics: 0,
        TimeDateStamp: 0,
        MajorVersion: 0,
        MinorVersion: 0,
        NumberOfNamedEntries: NamedEntries,
        NumberOfIdEntries: IdEntries
    };
}

pub unsafe fn Res2Size(refRes: &Base::Resource, num : usize) -> usize
{
    let json = serde_json::to_string_pretty(refRes).unwrap();
    Base::DebugOutput(format!("{}",json));
    if num != 0
    {
        //println!("{:?}",refRes);
        let mut size = 0usize;
        let mut i = 0usize;
        while i < (*refRes).Nextptr.len()
        {
            
            Base::DebugOutput(format!("num : {:x}",num));
            Base::DebugOutput(format!("i   : {:x}",i));
            Base::DebugOutput(format!("    : {:?}", (*refRes).Nextptr[i].IsData));
            
            
            if (*refRes).Nextptr[i].IsData
            {
                size += Data2Size();
            }
            else
            {
                size += Res2Size((*refRes).Nextptr[i].Res.as_ref(), num-1);
            }
            i += 1;
        }
        return size;
    }
    else
    {
        let json = serde_json::to_string_pretty(refRes).unwrap();
        //println!("{}",json);
        let mut size = size_of::<IMAGE_RESOURCE_DIRECTORY>();
        let numofent = refRes.Nextptr.len();
        size += numofent * size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>();
        return size;
    }
}

pub unsafe fn Data2Size() -> usize
{
    size_of::<IMAGE_RESOURCE_DATA_ENTRY>()
}