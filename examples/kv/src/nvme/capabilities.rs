use super::NvmeController;

impl NvmeController {
    fn cap_reserved_63_58(&self) -> u8 {
        ((self.read_cap() >> 58) & 0b11_1111) as u8
    }

    pub(crate) fn cap_cmbs(&self) -> bool {
        ((self.read_cap() >> 57) & 0b1) != 0
    }

    pub(crate) fn cap_pmbs(&self) -> bool {
        ((self.read_cap() >> 56) & 0b1) != 0
    }

    pub(crate) fn cap_mpsmax(&self) -> u8 {
        ((self.read_cap() >> 52) & 0b1111) as u8
    }

    pub(crate) fn cap_mpsmin(&self) -> u8 {
        ((self.read_cap() >> 48) & 0b1111) as u8
    }

    fn cap_reserved_47_46(&self) -> u8 {
        ((self.read_cap() >> 46) & 0b11) as u8
    }

    pub(crate) fn cap_bps(&self) -> bool {
        ((self.read_cap() >> 45) & 0b1) != 0
    }

    pub(crate) fn cap_css(&self) -> u8 {
        ((self.read_cap() >> 37) & 0b1111_1111) as u8
    }

    pub(crate) fn cap_nssrs(&self) -> bool {
        ((self.read_cap() >> 36) & 0b1) != 0
    }

    pub(crate) fn cap_dstrd(&self) -> u8 {
        ((self.read_cap() >> 32) & 0b1111) as u8
    }

    pub(crate) fn cap_to(&self) -> u8 {
        ((self.read_cap() >> 24) & 0b1111_1111) as u8
    }

    fn cap_reserved_23_19(&self) -> u8 {
        ((self.read_cap() >> 19) & 0b1_1111) as u8
    }

    // TODO: Do this... differently. I dont like it
    pub(crate) fn cap_ams(&self) -> (bool, bool) {
        // Weighted Round Robin with Urgent Priority Class
        let wrrups = ((self.read_cap() >> 18) & 0b1) != 0;

        // TODO: do i have anything that sets this bit?
        // Vendor specific bit likely meant to enable a vendor specific arbitration mechanism
        let vendor = ((self.read_cap() >> 17) & 0b1) != 0;

        (wrrups, vendor)
    }

    pub(crate) fn cap_cqr(&self) -> bool {
        ((self.read_cap() >> 16) & 0b1) != 0
    }

    pub(crate) fn cap_mqes(&self) -> u16 {
        ((self.read_cap() >> 0) & 0b1111_1111_1111_1111) as u16
    }

    pub(crate) fn print_caps_table(&self) {
        println!("+-----------------------------------------------------+");
        println!("| NVMe Capabilities                                   |");
        println!("+--------+-------+------------------------------------+");
        println!("| Name   | Value | Description                        |");
        println!("+--------+-------+------------------------------------+");

        print_table_row("CMBS", self.cap_cmbs(), "Controller Memory Buffer Supported");
        print_table_row("PMBS", self.cap_pmbs(), "Persistent Memory Region Supported");
        print_table_row("MPSMAX", self.cap_mpsmax(), "Memory Page Size Maximum");
        print_table_row("MPSMIN", self.cap_mpsmin(), "Memory Page Size Minimum");
        print_table_row("BPS", self.cap_bps(), "Boot Partition Support");

        // TODO: CSS
        // print_table_row("CSS", self.cap_css(), "Command Sets Supported");
        print_table_row("NSSRS", self.cap_nssrs(), "NVM Subsystem Reset Supported");
        print_table_row("DSTRD", self.cap_dstrd(), "Doorbell Stride");
        print_table_row("TO", self.cap_to(), "Timeout (500ms units)");

        // TODO: print small bit table somehow?
        //let (ams_wrrups, ams_vendor) = self.cap_ams();
        //print_table_row(
        //    "AMS (WRRUP)",
        //    if ams_wrrups { "Y" } else { "N" },
        //    "Weighted Round Robin with Urgent Priority Class",
        //);
        //print_table_row(
        //    "AMS (Vendor)",
        //    if ams_vendor { "Y" } else { "N" },
        //    "Vendor Specific",
        //);

        print_table_row("CQR", self.cap_cqr(), "Contiguous Queues Required");
        print_table_row("MQES", self.cap_mqes(), "Maximum Queue Entries Supported");
        println!("+--------+-------+------------------------------------+");
    }
}

fn print_table_row<T: ToString>(name: &str, value: T, description: &str) {
    // {:<6} is left aligned with at least 6 chars. Shorter values are padded
    println!("| {:<6} | {:>5} | {:<34} |", name, value.to_string(), description);
}
