mod nvme;
mod vfio;
use anyhow::Result;
use nvme::NvmeController;
use vfio::VfioContainer;

fn main() -> Result<()> {
    let vfio_container = VfioContainer::new()?;
    let (device, region_info) = vfio_container.open_device("/dev/vfio/42", "0000:02:00.0")?;
    let controller = NvmeController::new(device, region_info.size, region_info.offset)?;
    controller.print_spec_version()?;
    controller.print_caps_table()?;
    controller.setup_admin_queues()?;
    controller.enable_controller()?;
    println!("thats all folks");
    Ok(())
}
