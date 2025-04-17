mod parser;
use anyhow::Result;

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

fn main() -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        //.open("/usr/share/hwdata/pci.ids")?;
        .open("/tmp/pci.ids")?;
    let mmap_file = unsafe { memmap2::Mmap::map(&file)? };
    let input = std::str::from_utf8(&mmap_file)?;
    let (input, pci_ids) = PciIds::parse(input).unwrap();
    assert_eq!(input, "");
    dbg![pci_ids];
    Ok(())
}
