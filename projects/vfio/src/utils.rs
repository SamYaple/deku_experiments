use anyhow::{bail, Result};

#[derive(Debug)]
pub struct PciAddress {
    domain: u16,
    bus: u8,
    device: u8,
    function: u8,
}

impl PciAddress {
    pub fn new(bdf: &str) -> Result<Self> {
        let binding: Vec<_> = bdf.split(':').collect();
        let (dom, b, df) = match binding.as_slice() {
            [dom, b, df] => (dom, b, df),
            [b, df] => (&"0000", b, df),
            _ => bail! {format!{"Invalid bdf format -- '{bdf}'"}},
        };

        let binding: Vec<_> = df.split('.').collect();
        let (d, f) = match binding.as_slice() {
            [d, f] => (d, f),
            _ => bail! {format!{"Invalid bdf format -- '{bdf}'"}},
        };

        // TODO: These can still fail when the str is empty and the error message is not helpful
        let domain = u16::from_str_radix(dom, 16)?;
        let bus = u8::from_str_radix(b, 16)?;
        let device = u8::from_str_radix(d, 16)?;
        let function = u8::from_str_radix(f, 16)?;

        // TODO: check if this is defined in the PCI spec or somewhere else
        if device > 31 {
            bail! {format!{"device must be <= 31, we got '{device}'"}};
        }
        if function > 7 {
            bail! {format!{"device function must be <= 7, we got '{function}'"}};
        }

        Ok(Self { domain, bus, device, function })
    }
}

impl std::fmt::Display for PciAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{}", self.domain, self.bus, self.device, self.function)
    }
}
