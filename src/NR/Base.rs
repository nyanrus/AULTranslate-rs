use windows::Win32::{Foundation::{PSTR, HANDLE}, System::Diagnostics::Debug::{OutputDebugStringA}};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct ImgResDir
{
    pub count: usize,
    pub adr: usize,
}

#[derive(Clone, Debug)]
pub struct ImgResDirEnt
{
    pub IsId: bool,
    pub Id: u16,
    pub Name: String,
    pub NextIsDir: bool,
    pub add: usize,
}


pub struct NRTrait
{
    pub hHandle : HANDLE,
    pub ImageBase: usize,
    pub ResRVA: usize,
    pub Int: usize,
    pub ImgResDir3: [ImgResDir; 3],
    pub ImgResDirEnt3: [ImgResDirEnt; 3],
}

impl Default for ImgResDir
{
    fn default() -> Self
    {
        ImgResDir {
            count: Default::default(),
            adr: Default::default(),
        }
    }
}

impl Default for ImgResDirEnt
{
    fn default() -> Self
    {
        ImgResDirEnt {
            IsId: Default::default(),
            Id: Default::default(),
            Name: Default::default(),
            NextIsDir: Default::default(),
            add: Default::default(),
        }
    }
}

impl Default for NRTrait
{
    fn default() -> Self
    {
        NRTrait {
            hHandle : Default::default(),
            ImageBase: Default::default(),
            ResRVA: Default::default(),
            Int: Default::default(),
            ImgResDirEnt3: Default::default(),
            ImgResDir3: Default::default(),
        }
    }
}

#[allow(dead_code)]
pub unsafe fn DebugOutputstr(s: &str)
{
    DebugOutput(s.to_string());
}
pub unsafe fn DebugOutput(s: String)
{
    OutputDebugStringA(PSTR(format!("{} --", s).as_mut_ptr()));
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Resource
{
    pub NameId : Resource_0,
    pub Nextptr : Vec<Resource_1>,
}


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Resource_0
{
    pub IsName : bool,
    pub Id : u16,
    pub Name : String,
}


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Resource_1
{
    pub IsData : bool,
    pub Res : Option<Box<Resource>>,
    pub Data : Option<Box<Res_Data>>,
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Res_Data
{
    pub NameId : Resource_0,
    pub Data : Vec<u8>,
    pub CodePage : u32,
}


impl Default for Resource
{
    fn default() -> Self {
        Resource { NameId: Default::default(), Nextptr: Default::default() }
    }
}
impl Default for Resource_0
{
    fn default() -> Self {
        Resource_0{ IsName: Default::default(), Id: Default::default(), Name: Default::default() }
    }
}

impl Default for Resource_1
{
    fn default() -> Self {
        Resource_1{ IsData: Default::default(), Res: None, Data: None}
    }
}

impl Default for Res_Data
{
    fn default() -> Self {
        Res_Data { NameId: Default::default(), Data: Default::default(), CodePage: Default::default() }
    }
}

impl Resource
{
    pub fn new() -> Self
    {
        Resource{ NameId: Default::default(), Nextptr: Default::default() }
    }
}

pub struct pointerbylevel
{

}