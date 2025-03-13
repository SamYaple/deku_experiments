use anyhow::{anyhow, Result};

fn read_bar_address(pci_addr: &str) -> Result<(u64, u64)> {
    let path = format!("/sys/bus/pci/devices/{}/resource", pci_addr);
    let contents = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = contents.lines().collect();
    let bar_info = lines.get(0).ok_or_else(|| anyhow!{"BAR not found"})?;
    let mut parts = bar_info.split_whitespace();
    let start = u64::from_str_radix(parts.next().unwrap().trim_start_matches("0x"), 16)?;
    let end = u64::from_str_radix(parts.next().unwrap().trim_start_matches("0x"), 16)?;
    Ok((start, (end - start) + 1))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match read_bar_address(&args[1]) {
        Ok((addr, size)) => {
            println!("BAR0 addr: {:#x}", addr);
            println!("BAR0 size: {:#x}", size);
        }
        Err(e) => eprintln!("Failed to read BAR: {}", e),
    }
}
