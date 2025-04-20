mod dma;
use anyhow::Result;
use nvme::NvmeController;
use vfio::{VfioContainer, VfioGroup};
use pci::PciAddress;

fn main() -> Result<()> {
    eprintln!("Using hardcoded device path -- 02:00.0");
    let pci_address = &PciAddress::new("02:00.0")?;
    let group_id = VfioGroup::get_id_from_address(pci_address)?;
    let mut container = VfioContainer::new()?;
    let group = container.add_group(group_id)?;
    group.add_device(pci_address)?;

    //let group_status = group.get_status()?;
    //dbg![group_status.get_flags()];

    let device = group.get_device(pci_address)?;

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
