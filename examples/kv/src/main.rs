mod dma;
use anyhow::{bail, Result};
use nvme::NvmeController;
use vfio::{VfioContainer, VfioGroup, VfioDevice};
use pci::{PciAddress, PciDevice};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
use std::os::fd::AsRawFd;

fn pci_config_from_vfio_device(device: &VfioDevice) -> Result<PciDevice> {
    let region_info = device.get_region_info(7)?;
    let mut bytes = PciDevice::default().to_bytes()?;

    let ret = unsafe { libc::pread(device.as_raw_fd(), bytes.as_mut_ptr() as *mut _, PciDevice::SERIALIZED_BYTE_SIZE, region_info.get_offset() as i64) };
    if ret < 0 {
        bail! {std::io::Error::last_os_error()};
    }

    let ((_, remaining), pci_device) = PciDevice::from_bytes((&bytes, 0))?;
    debug_assert!(remaining == 0);
    Ok(pci_device)
}

fn main() -> Result<()> {
    let pci_address = &PciAddress::new("02:00.0")?;
    let group_id = VfioGroup::get_id_from_address(pci_address)?;

    let mut container = VfioContainer::new()?;
    let group = container.add_group(group_id)?;
    group.add_device(pci_address)?;

    //let group_status = group.get_status()?;
    //dbg![group_status.get_flags()];

    let device = group.get_device(pci_address)?;
    let pci_device_from_vfio = pci_config_from_vfio_device(&device)?;
    let pci_device_from_sysfs = PciDevice::new(pci_address)?;
    assert_eq!(pci_device_from_vfio, pci_device_from_sysfs);


    //let device_info = device.get_device_info()?;
    //dbg![device_info.get_flags()];

    let mut controller = NvmeController::new(device)?;
    controller.print_spec_version()?;
    controller.print_caps_table()?;

    print!("Enabling controller... ");
    controller.enable_controller()?;
    controller.wait_for_controller_ready()?;
    println!("Successful!");

    print!("Telling controller to shutdown... ");
    controller.shutdown_controller()?;
    controller.wait_for_controller_shutdown()?;
    println!("Successful!");

    print!("Disabling controller... ");
    controller.disable_controller()?;
    controller.wait_for_controller_stop()?;
    println!("Successful!");

    println!("Sleeping for 30 seconds (it is safe to ctrl-c)....");
    std::thread::sleep(std::time::Duration::from_secs(30));
    println!("thats all folks");
    Ok(())
}
