mod parser;
use anyhow::Result;
use memmap2::Mmap;
use std::fs::OpenOptions;

#[derive(Debug, PartialEq)]
pub struct PciIds {
    pub classes: Vec<Class>,
    pub vendors: Vec<Vendor>,
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub id: u8,
    pub name: String,
    pub subclasses: Vec<SubClass>,
}

#[derive(Debug, PartialEq)]
pub struct SubClass {
    pub id: u8,
    pub name: String,
    pub prog_ifs: Vec<ProgIf>,
}

#[derive(Debug, PartialEq)]
pub struct ProgIf {
    pub id: u8,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Vendor {
    pub id: u16,
    pub name: String,
    pub devices: Vec<Device>,
}

#[derive(Debug, PartialEq)]
pub struct Device {
    pub id: u16,
    pub name: String,
    pub subsystems: Vec<Subsystem>,
}

#[derive(Debug, PartialEq)]
pub struct Subsystem {
    pub subvendor_id: u16,
    pub subdevice_id: u16,
    pub name: String,
}

pub fn load_from_file() -> Result<PciIds> {
    let file = OpenOptions::new()
        .read(true)
        .open("/usr/share/hwdata/pci.ids")?;
    let mmap_file = unsafe { Mmap::map(&file)? };
    let input = std::str::from_utf8(&mmap_file)?;
    // It was absolutely not neccesary to mmap this file. We could have read it into a String.
    // This seemed more fun though...

    let (input, pci_ids) = PciIds::parse(input).unwrap();
    assert_eq!(input, "");
    Ok(pci_ids)
}
