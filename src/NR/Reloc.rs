//use super::*;
use cty::*;
use windows::Win32::System::SystemServices::IMAGE_BASE_RELOCATION;

pub unsafe fn ReadReloc(pReloc: &mut usize, mut vec: Box<Vec<usize>>)
{
    println!("{:x}",*pReloc as usize);
    let pImgBaseReloc = *pReloc as *const IMAGE_BASE_RELOCATION;
    let mut pU16 = *pReloc as *mut u16;
    while (pU16 as usize) < (*pReloc + (*pImgBaseReloc).SizeOfBlock as usize)
    {
        if *pU16 >> 3 == 3
        {
            println!("in while : {:x}",(*pImgBaseReloc).VirtualAddress as usize + *pU16 as usize);
            vec.push((*pImgBaseReloc).VirtualAddress as usize + (*pU16 & 0x0fff) as usize);
        }
        pU16 = pU16.offset(1);
    }
    *pReloc += (*pImgBaseReloc).SizeOfBlock as usize;
}

pub unsafe fn ReadRelocAll(mut pReloc : usize) -> Vec<usize>
{
    println!("all : {:x}",pReloc);
    let vec = Vec::<usize>::new();
    let vecBox = Box::new(vec);
    loop
    {
        println!("if : {:x}",*(pReloc as *const usize));
        if *(pReloc as *const usize) != 0
        {
            ReadReloc(&mut pReloc, vecBox.clone());
        }
        else
        {
            break;
        }
    }
    *vecBox
}