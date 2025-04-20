use anyhow::Result;
use pci::{PciAddress, PciDevice};

fn main() -> Result<()> {
    let base_path = std::path::Path::new("/sys/bus/pci/devices");
    for entry in std::fs::read_dir(base_path)? {
        let entry = entry?;
        if let Some(address) = entry.file_name().to_str() {
            let pci_address = &PciAddress::new(address)?;
            let Ok(pci_device) = PciDevice::new(pci_address) else {
                eprintln!["skipping on parse failure: {}", address];
                continue;
            };
            dbg![pci_device];
        }
    }
    Ok(())
}
