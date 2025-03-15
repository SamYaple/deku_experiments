mod nvme;
mod vfio;
use anyhow::Result;
use nvme::NvmeController;
use vfio::VfioContainer;

fn main() -> Result<()> {
    let vfio_container = VfioContainer::new()?;
    let device = vfio_container.open_device("/dev/vfio/42", "0000:02:00.0")?;
    let mut controller = NvmeController::new(device)?;
    controller.print_caps_table();
    Ok(())
}
