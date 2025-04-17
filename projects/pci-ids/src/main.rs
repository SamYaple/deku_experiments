mod parser;
mod search;
use anyhow::Result;
use memmap2::Mmap;
use std::fs::OpenOptions;

#[derive(Debug, PartialEq)]
pub struct PciIds<'a> {
    classes: Vec<Class<'a>>,
    vendors: Vec<Vendor<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Class<'a> {
    id: u8,
    name: &'a str,
    subclasses: Vec<SubClass<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct SubClass<'a> {
    id: u8,
    name: &'a str,
    prog_ifs: Vec<ProgIf<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ProgIf<'a> {
    id: u8,
    name: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Vendor<'a> {
    id: u16,
    name: &'a str,
    devices: Vec<Device<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Device<'a> {
    id: u16,
    name: &'a str,
    subsystems: Vec<Subsystem<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Subsystem<'a> {
    subvendor_id: u16,
    subdevice_id: u16,
    name: &'a str,
}

#[derive(Debug)]
pub enum PciEntity<'a> {
    Class(&'a Class<'a>),
    SubClass(&'a SubClass<'a>),
    ProgIf(&'a ProgIf<'a>),
    Vendor(&'a Vendor<'a>),
    Device(&'a Device<'a>),
    Subsystem(&'a Subsystem<'a>),
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("/usr/share/hwdata/pci.ids")?;
    let mmap_file = unsafe { Mmap::map(&file)? };
    let input = std::str::from_utf8(&mmap_file)?;
    let (input, pci_ids) = PciIds::parse(input).unwrap();
    assert_eq!(input, "");
    let ret = pci_ids.search("10f0");
    dbg![ret];
    Ok(())
}
