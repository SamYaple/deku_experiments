use super::NvmeController;
impl NvmeController {
    pub(crate) fn get_spec_version(&self) -> (u16, u8, u8) {
        let mjr = ((self.read_vs() >> 16) & 0b1111_1111_1111_1111) as u16;
        let mnr = ((self.read_vs() >> 8)  & 0b1111_1111) as u8;
        let ter = ((self.read_vs() >> 0)  & 0b1111_1111) as u8;
        (mjr, mnr, ter)
    }

    pub(crate) fn print_spec_version(&self) -> () {
        let (mjr, mnr, ter) = self.get_spec_version();
        println!("NVMe spec version: {}.{}.{}", mjr, mnr, ter);
    }
}
