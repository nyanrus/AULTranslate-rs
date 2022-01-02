use core::mem::size_of;
use windows::Win32::System::Diagnostics::Debug::IMAGE_SECTION_HEADER;
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
) //-> Base::Resource
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
    //refRes
    /*
    let a = (*pNRTrait).Int;
    if (*pNRTrait).ImgResDir3[a].adr == 0
    {
        (*pNRTrait).ImgResDir3[a].adr = pImgResDir as usize;
    } else if a == 2
    {
        (*pNRTrait).ImgResDir3[a].adr = pImgResDir as usize;
    }

    let mut count = 0usize;
    let NumOfIdEnt = (*pImgResDir).NumberOfIdEntries as usize;
    let NumOfNameEnt = (*pImgResDir).NumberOfNamedEntries as usize;
    let pImgResDirEnt = NextIRDir(pImgResDir as usize) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;

    while count < NumOfIdEnt + NumOfNameEnt {
        println!("\n  R_pIRD : {:x}", pImgResDir as usize);
        println!("a : {:x}", a);
        println!("numof~~ : {:x}", NumOfIdEnt + NumOfNameEnt);
        println!("count : {:x}", count);
        println!("imgresdir : {:?}", (*pNRTrait).ImgResDir3);
        println!("Int : {:x}", (*pNRTrait).Int);

        ReadImgResDirEnt(
            (pImgResDirEnt as usize + count * size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>())
                as *const IMAGE_RESOURCE_DIRECTORY_ENTRY,
            pNRTrait,
            pVec,
            refResource,
        );

        count += 1;
        (*pNRTrait).ImgResDir3[a].count = count;
        //pImgResDirEnt = pImgResDirEnt.offset(1);
    }
    println!("out Int : {:x}", (*pNRTrait).Int);
    if (*pNRTrait).Int > 0
    {
        (*pNRTrait).Int -= 1;
        (*pNRTrait).ImgResDir3[(*pNRTrait).Int].adr =
            pImgResDirEnt as usize + count * size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>();
    }
    */
}

/*
unsafe fn IsDir(pImgResDirEnt: *const IMAGE_RESOURCE_DIRECTORY_ENTRY) -> bool
{
    (*pImgResDirEnt).Anonymous2.Anonymous._bitfield & 0x80000000 == 0x80000000
}

#[deprecated]
#[allow(dead_code)]
unsafe fn IsName(
    hHandle : HANDLE,
    pImgResDirEnt: *const IMAGE_RESOURCE_DIRECTORY_ENTRY,
    resVA: usize,
    refResource : &mut Box<Base::Resource>,
) -> Base::ImgResDirEnt
{
    println!("pImgResDirEnt : {:x}", pImgResDirEnt as usize);
    println!(
        "*pImgResDirEnt.a : {:x}",
        (*pImgResDirEnt).Anonymous1.Anonymous._bitfield as usize
    );
    println!(
        "*pImg~~ : {:x}",
        ((*pImgResDirEnt).Anonymous1.Anonymous._bitfield & 0x0000FFFF)
    );

    let refNameId = &mut ((*refResource).NameId);

    if (*pImgResDirEnt).Anonymous1.Anonymous._bitfield & 0x80000000 == 0x80000000
    {
        let mut i = 0u16;
        let pImgResDirStrU = (resVA + ((*pImgResDirEnt).Anonymous1.Name & 0x7FFFFFFF) as usize)
            as *const IMAGE_RESOURCE_DIR_STRING_U;

        let mut Buf =
            Vec::<u16>::with_capacity(((*pImgResDirStrU).Length as usize) * size_of::<u16>());
        Base::DebugOutput(
            format!(
                "    pImgResDirStrU : {:x}\n    Length : {:x}",
                pImgResDirStrU as usize,
                (*pImgResDirStrU).Length
            )
        );
        let mut ptr = (pImgResDirStrU as usize + size_of::<u16>()) as *const u16;
        while i < (*pImgResDirStrU).Length
        {
            Base::DebugOutput(format!("{:x}",ptr as usize));
            Buf.push(*ptr);
            ptr = ptr.offset(1);
            i += 1;
        }
        Base::DebugOutput(format!("Buf : {:?}", Buf.clone()));
        (*refNameId).IsName = true;
        (*refNameId).Name = String::from_utf16_lossy(&Buf);
        return Base::ImgResDirEnt {
            IsId: false,
            Name: String::from_utf16_lossy(&Buf),
            ..Default::default()
        };
    }
    else
    {
        (*refNameId).IsName = false;
        (*refNameId).Id = ((*pImgResDirEnt).Anonymous1.Anonymous._bitfield & 0x0000FFFF) as u16;
        return Base::ImgResDirEnt {
            IsId: true,
            Id: ((*pImgResDirEnt).Anonymous1.Anonymous._bitfield & 0x0000FFFF) as u16,
            ..Default::default()
        };
    }
}
*/

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
    /*
    // if nameid == name(utf16_string)
    (*pNRTrait).ImgResDirEnt3[(*pNRTrait).Int] =
        IsName((*pNRTrait).hHandle,pImgResDirEnt, (*pNRTrait).ImageBase + (*pNRTrait).ResRVA, refResource);

    let refNextptr = &mut ((*refResource).Nextptr);
    // if next is resdir
    if IsDir(pImgResDirEnt) {
        println!("3");
        (*refNextptr).push(Base::Resource_1{
            IsData: false,
            ..Default::default()
        });
        (*pNRTrait).Int += 1;
        (*pNRTrait).ImgResDirEnt3[(*pNRTrait).Int].NextIsDir = true;
        let len = (*refNextptr).len();
        ReadImgResDir(
            ((*pNRTrait).ImageBase
                + (*pNRTrait).ResRVA
                + ((*pImgResDirEnt).Anonymous2.OffsetToData & 0x7fffffff) as usize)
                as *const IMAGE_RESOURCE_DIRECTORY,
            pNRTrait,
            pVec,
            (*refNextptr)[len-1].Res.as_mut(),
        );
    }
    // else if next is resdata
    else {
        println!("4");
        (*pNRTrait).ImgResDirEnt3[(*pNRTrait).Int].add = (*pNRTrait).ImageBase
            + (((*pImgResDirEnt).Anonymous2.OffsetToData & 0x7fffffff) as usize) as usize;
        (*pVec).push((*pNRTrait).ImgResDirEnt3.clone());
    }
    */
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