mod nvme;
mod dma;
mod vfio;
use anyhow::Result;
use nvme::NvmeController;
use vfio::VfioContainer;

fn main() -> Result<()> {
    //dbg![std::mem::size_of::<crate::nvme::Command>()];
    //panic!();

    let vfio_container = VfioContainer::new()?;
    let (device, region_info) = vfio_container.open_device("/dev/vfio/42", "0000:02:00.0")?;
    let mut controller = NvmeController::new(device, region_info.size, region_info.offset)?;
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
