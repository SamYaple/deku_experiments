mod dma;
mod nvme;
use anyhow::Result;
use nvme::NvmeController;
use vfio::{VfioContainer, PciAddress};

fn main() -> Result<()> {
    let mut container = VfioContainer::new()?;
    let group = container.add_group(42)?;
    let device = group.add_device(PciAddress::new("02:00.0")?)?;

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

    println!("thats all folks");
    Ok(())
}
