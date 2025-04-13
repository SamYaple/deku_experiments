pub mod container;
pub use container::VfioContainer;

pub mod group;
pub use group::VfioGroup;

pub mod device;
pub use device::VfioDevice;

pub mod utils;
pub use utils::PciAddress;

// VFIO definitions (from linux/vfio.h and friends)
// TODO: generate this in some fun way
const VFIO_API_VERSION_EXPECTED: u32 = 0;
const VFIO_IOMMU_TYPE1V2: u32 = 3;

const VFIO_TYPE: u32 = 0b0011_1011_0000_0000;

const VFIO_GET_API_VERSION_IOCTL:  u64 = (VFIO_TYPE | 100) as u64;
const VFIO_CHECK_EXTENSION_IOCTL:  u64 = (VFIO_TYPE | 101) as u64;
const VFIO_SET_IOMMU_IOCTL:        u64 = (VFIO_TYPE | 102) as u64;
const VFIO_GROUP_GET_STATUS:       u64 = (VFIO_TYPE | 103) as u64;
const VFIO_GROUP_SET_CONTAINER:    u64 = (VFIO_TYPE | 104) as u64;
const VFIO_GROUP_GET_DEVICE_FD:    u64 = (VFIO_TYPE | 106) as u64;
const VFIO_DEVICE_GET_REGION_INFO: u64 = (VFIO_TYPE | 108) as u64;
