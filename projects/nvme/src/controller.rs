use super::NvmeController;
use anyhow::{bail, Result};
use deku::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 2, id_type = "u8")]
pub(crate) enum ShutdownNotification {
    #[deku(id = 0b00)]
    Noop,
    #[deku(id = 0b01)]
    Normal,
    #[deku(id = 0b10)]
    Abrupt,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 3, id_type = "u8")]
pub(crate) enum ArbitrationMechanismSelected {
    #[deku(id = 0b000)]
    RoundRobin,
    #[deku(id = 0b001)]
    WeightedRoundRobinWithUrgentPriorityClass,
    #[deku(id = 0b111)]
    VendorSpecific,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 3, id_type = "u8")]
pub(crate) enum CommandSetSelected {
    #[deku(id = 0b000)]
    NVMCommandSet,
    #[deku(id = 0b111)]
    AdminCommandSetOnly,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub(crate) struct ControllerConfiguration {
    #[deku(bits = 8)]
    _reserved_31_24: u8,
    #[deku(bits = 4)]
    iocqes: u8,
    #[deku(bits = 4)]
    iosqes: u8,
    shn: ShutdownNotification,
    ams: ArbitrationMechanismSelected,
    #[deku(bits = 4)]
    mps: u8,
    css: CommandSetSelected,
    #[deku(bits = 3)]
    _reserved_03_01: u8,
    #[deku(bits = 1)]
    en: bool,
}

#[derive(Debug, DekuRead, DekuWrite, PartialEq)]
#[deku(bits = 2, id_type = "u8")]
pub(crate) enum ShutdownStatus {
    #[deku(id = 0b00)]
    NormalOperation,
    #[deku(id = 0b01)]
    ShutdownOccuring,
    #[deku(id = 0b10)]
    ShutdownComplete,
}

impl std::fmt::Display for ShutdownStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NormalOperation => write!(f, "Normal  Operation"),
            Self::ShutdownOccuring => write!(f, "Shutdown Occuring"),
            Self::ShutdownComplete => write!(f, "Shutdown Complete"),
        }
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub(crate) struct ControllerStatus {
    #[deku(bits = 26)]
    _reserved_31_06: u32,
    #[deku(bits = 1)]
    pp: bool,
    #[deku(bits = 1)]
    nssro: bool,
    pub(crate) shst: ShutdownStatus,
    #[deku(bits = 1)]
    cfs: bool,
    #[deku(bits = 1)]
    pub(crate) rdy: bool,
}

impl ControllerConfiguration {
    pub(crate) fn from_raw(val: u32) -> Result<Self> {
        let bytes = val.to_be_bytes();
        let ((_, remaining), version) = Self::from_bytes((&bytes, 0))?;
        if remaining > 0 {
            bail! {"failed to consume all data"};
        }
        Ok(version)
    }
}

impl Into<u32> for ControllerConfiguration {
    fn into(self) -> u32 {
        // TODO: think this through. It might be fine, I just have no checks or tests around it
        let bytes = self.to_bytes().unwrap();
        u32::from_be_bytes(bytes[..4].try_into().unwrap())
    }
}

impl ControllerStatus {
    pub(crate) fn from_raw(val: u32) -> Result<Self> {
        let bytes = val.to_be_bytes();
        let ((_, remaining), version) = Self::from_bytes((&bytes, 0))?;
        if remaining > 0 {
            bail! {"failed to consume all dataation"};
        }
        Ok(version)
    }
}

impl Into<u32> for ControllerStatus {
    fn into(self) -> u32 {
        // TODO: think this through. It might be fine, I just have no checks or tests around it
        let bytes = self.to_bytes().unwrap();
        u32::from_be_bytes(bytes[..4].try_into().unwrap())
    }
}

impl NvmeController<'_> {
    pub(crate) fn get_controller_configuration(&self) -> Result<ControllerConfiguration> {
        let val = unsafe { std::ptr::read_volatile(&self.registers.as_ref().cc) };
        ControllerConfiguration::from_raw(val)
    }

    pub(crate) fn get_controller_status(&self) -> Result<ControllerStatus> {
        let val = unsafe { std::ptr::read_volatile(&self.registers.as_ref().csts) };
        ControllerStatus::from_raw(val)
    }

    pub(crate) fn write_controller_configuration(
        &mut self,
        cc: ControllerConfiguration,
    ) -> Result<()> {
        let val: u32 = cc.into();
        unsafe {
            std::ptr::write_volatile(&mut self.registers.as_mut().cc, val);
        };
        Ok(())
    }

    pub fn enable_controller(&mut self) -> Result<()> {
        let mut cc = self.get_controller_configuration()?;
        if cc.en {
            // TODO: this screams trait based state solution at my brain. or something else
            bail!("Controller is already enabled; refusing to enable twice");
        }
        cc.iocqes = 4;
        cc.iosqes = 4;
        cc.en = true;
        self.write_controller_configuration(cc)
    }

    pub fn ready(&self) -> Result<bool> {
        let status = self.get_controller_status()?;
        Ok(status.rdy)
    }

    pub fn wait_for_controller_ready(&self) -> Result<()> {
        // TODO use reported timeout from device
        let mut timeout = 100;
        while timeout > 0 {
            if self.ready()? {
                return Ok(());
            }
            sleep(Duration::from_millis(10));
            timeout -= 1;
        }
        bail! {"Timeout waiting for NVMe controller to become ready"};
    }

    pub fn wait_for_controller_stop(&self) -> Result<()> {
        // TODO use reported timeout from device
        let mut timeout = 100;
        while timeout > 0 {
            if !self.ready()? {
                return Ok(());
            }
            sleep(Duration::from_millis(10));
            timeout -= 1;
        }
        bail! {"Timeout waiting for NVMe controller to stop"};
    }
    pub fn wait_for_controller_shutdown(&self) -> Result<()> {
        let mut timeout = 100;
        while timeout > 0 {
            let status = self.get_controller_status()?;
            if status.shst == ShutdownStatus::ShutdownComplete {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
            timeout -= 1;
        }
        bail! {"Timeout waiting for NVMe controller to shutdown"};
    }

    pub fn shutdown_controller(&mut self) -> Result<()> {
        let mut cc = self.get_controller_configuration()?;
        cc.shn = ShutdownNotification::Normal;
        self.write_controller_configuration(cc)
    }

    pub fn disable_controller(&mut self) -> Result<()> {
        let mut cc = self.get_controller_configuration()?;
        if !cc.en {
            bail!("Controller is already disabled; refusing to disable twice");
        }
        cc.en = false;
        self.write_controller_configuration(cc)
    }
}
