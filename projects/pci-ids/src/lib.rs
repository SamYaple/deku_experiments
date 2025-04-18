mod parser;

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

pub fn load_from_file(p: &std::path::Path) -> std::io::Result<PciIds> {
    let pci_ids_file_string = std::fs::read_to_string(p)?;
    let (input, pci_ids) = PciIds::parse(&pci_ids_file_string).unwrap();
    assert_eq!(input, "");
    Ok(pci_ids)
}
