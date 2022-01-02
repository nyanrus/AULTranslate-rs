//use super::*;
use windows::Win32::System::SystemServices::IMAGE_BASE_RELOCATION;

pub unsafe fn ReadReloc(pReloc: &mut usize) -> Vec<usize>
{
    let mut vec = Vec::<usize>::new();
    //println!("{:x}",*pReloc as usize);
    let pImgBaseReloc = *pReloc as *const IMAGE_BASE_RELOCATION;
    //println!("pimg~~ : {:x}", pImgBaseReloc as usize);
    //println!("pimgbasereloc : {:x}, {:x}", (*pImgBaseReloc).VirtualAddress, (*pImgBaseReloc).SizeOfBlock);
    let mut pU16 = *pReloc as *mut u16;
    while (pU16 as usize) < (*pReloc + (*pImgBaseReloc).SizeOfBlock as usize)
    {
        //println!("pU16 : {:x}, >> : {:x}",*pU16, *pU16 >> 12);
        if *pU16 >> 12 == 3
        {
            //println!("in while : {:x}",(*pImgBaseReloc).VirtualAddress as usize + *pU16 as usize);
            vec.push((*pImgBaseReloc).VirtualAddress as usize + (*pU16 & 0x0fff) as usize);
        }
        pU16 = pU16.offset(1);
    }
    *pReloc += (*pImgBaseReloc).SizeOfBlock as usize;
    vec
}

pub unsafe fn ReadRelocAll(mut pReloc : usize) -> Vec<usize>
{
    println!("all : {:x}",pReloc);
    let mut vec = Vec::<usize>::new();
    loop
    {
        //println!("if : {:x}",*(pReloc as *const usize));
        if *(pReloc as *const usize) != 0
        {
            let mut tmp = ReadReloc(&mut pReloc);
            vec.append(&mut tmp);
        }
        else
        {
            break;
        }
    }
    vec
}