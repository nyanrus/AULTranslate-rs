use core::mem::size_of;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_SECTION_HEADER, IMAGE_DATA_DIRECTORY};
use windows::Win32::System::SystemServices::{IMAGE_RESOURCE_DIRECTORY, IMAGE_RESOURCE_DATA_ENTRY};
use windows::Win32::System::SystemServices::{
    IMAGE_RESOURCE_DIRECTORY_ENTRY, IMAGE_RESOURCE_DIR_STRING_U,
};

use super::*;

#[allow(dead_code)]
unsafe fn NextIRDir(u: usize) -> usize
{
    return u + size_of::<IMAGE_RESOURCE_DIRECTORY>();
}

pub unsafe fn ReadImgResDirn
(
    pImgResDir : *const IMAGE_RESOURCE_DIRECTORY,
    refNRTrait : &mut Base::NRTrait,
    refRes : &mut Base::Resource,
)
{
    let num_of_ent = (*pImgResDir).NumberOfIdEntries as usize + (*pImgResDir).NumberOfNamedEntries as usize;
    let mut i = 0usize;
    let mut pImgResDirEnt = pImgResDir.offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
    while i < num_of_ent
    {
        ReadImgResDirEntn(pImgResDirEnt, refNRTrait, refRes);
        pImgResDirEnt = pImgResDirEnt.offset(1);
        i += 1;
    }
}

pub unsafe fn ReadImgResDirEntn
(
    pImgResDirEnt : *const IMAGE_RESOURCE_DIRECTORY_ENTRY,
    refNRTrait : &mut Base::NRTrait,
    refRes : &mut Base::Resource,
)
{
    let analyz = AnalyzeImgResDirEnt(pImgResDirEnt);
    let NameId = NameId(&analyz, refNRTrait);
    if analyz.IsDir
    {
        let mut pImgResDir = analyz.DirDat as usize;
        pImgResDir += refNRTrait.ImageBase + refNRTrait.ResRVA;
        let pImgResDir = pImgResDir as *const IMAGE_RESOURCE_DIRECTORY;
        refRes.Nextptr.push(Base::Resource_1 { IsData: false, ..Default::default()});
        let len = refRes.Nextptr.len();
        refRes.Nextptr[len-1].Res.NameId = NameId;
        ReadImgResDirn(pImgResDir, refNRTrait, refRes.Nextptr[len-1].Res.as_mut());
    }
    else
    {
        let mut pDat = analyz.DirDat as usize;
        pDat += refNRTrait.ImageBase + refNRTrait.ResRVA;
        let pImgResDataEnt = pDat as *const IMAGE_RESOURCE_DATA_ENTRY;
        let pDat = (*pImgResDataEnt).OffsetToData as usize + refNRTrait.ImageBase;
        let mut pDat = pDat as *const u8;

        let mut i = 0u32;
        let mut vec = Vec::<u8>::new();
        while i < (*pImgResDataEnt).Size
        {
            vec.push(*pDat);
            pDat = pDat.offset(1);
            i += 1;
        }
        refRes.Nextptr.push(
            Base::Resource_1 { IsData: true,
            Data: Box::new(
                Base::Res_Data{
                    NameId,
                    Data: vec,
                CodePage: (*pImgResDataEnt).CodePage,
            }),
        ..Default::default()});
    }
}

pub unsafe fn AnalyzeImgResDirEnt
(
    pImgResDirEnt : *const IMAGE_RESOURCE_DIRECTORY_ENTRY,
) -> stAnalyzImgResDirEnt
{
    let IsName = (*pImgResDirEnt).Anonymous1.Anonymous._bitfield & 0x80000000 != 0;
    let NameId = (*pImgResDirEnt).Anonymous1.Anonymous._bitfield & 0x7fffffff;
    let IsDir = (*pImgResDirEnt).Anonymous2.Anonymous._bitfield & 0x80000000 != 0;
    let DirDat = (*pImgResDirEnt).Anonymous2.Anonymous._bitfield & 0x7fffffff;

    return stAnalyzImgResDirEnt { IsName, NameId, IsDir, DirDat };
}

pub struct stAnalyzImgResDirEnt
{
    pub IsName : bool,
    pub NameId : u32,
    pub IsDir : bool,
    pub DirDat : u32,
}

pub unsafe fn NameId
(
    refstAnalyzImgResDirEnt : &stAnalyzImgResDirEnt,
    refNRTrait : &mut Base::NRTrait,
) -> Base::Resource_0
{
    let st = refstAnalyzImgResDirEnt;

    let IsName = st.IsName;
    let mut Name = String::new();
    let mut Id = 0u16;
    if st.IsName
    {
        let adr = refNRTrait.ImageBase + refNRTrait.ResRVA + st.NameId as usize;
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
        Name = String::from_utf16_lossy(&u16vec);
    }
    else
    {
        Id = st.NameId as u16;
    }
    return Base::Resource_0 { IsName, Id, Name };
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
            println!("CodePage : {}",refRes.Nextptr[i].Data.CodePage);
            println!("Data : {:?}",refRes.Nextptr[i].Data.Data);
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


/*
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
        println!("1");
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
        println!("2");
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
        println!("3");
        ReadImgResDir(adr as *const IMAGE_RESOURCE_DIRECTORY, refNRTrait, refVec, refRes);
    }
    else
    {
        println!("4");
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
        println!("4-fin");
        let Dat = Base::Res_Data{ NameId: refRes.NameId.clone(), Data: vec.clone(), CodePage: (*a).CodePage };
        refRes.Data = Base::Resource_1 { IsData: true, Data: Box::new(Dat), ..Default::default()};
    }
}
*/