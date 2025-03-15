use anyhow::{bail, Result};
use std::sync::Arc;
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::os::fd::{FromRawFd, AsRawFd};
use std::io;

// VFIO definitions (from linux/vfio.h and friends)
// TODO: generate this in some fun way
const VFIO_API_VERSION_EXPECTED: u32 = 0;
const VFIO_GROUP_FLAGS_VIABLE: u32 = 1;
const VFIO_IOMMU_TYPE1V2: u32 = 3;

const VFIO_TYPE: u32 = b';' as u32;
const VFIO_BASE: u32 = 100;

const VFIO_GET_API_VERSION_IOCTL:  u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 0)) as u64;
const VFIO_CHECK_EXTENSION_IOCTL:  u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 1)) as u64;
const VFIO_SET_IOMMU_IOCTL:        u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 2)) as u64;
const VFIO_GROUP_GET_STATUS:       u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 3)) as u64;
const VFIO_GROUP_SET_CONTAINER:    u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 4)) as u64;
const VFIO_GROUP_GET_DEVICE_FD:    u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 6)) as u64;
const VFIO_DEVICE_GET_REGION_INFO: u64 = ((VFIO_TYPE << 8) | (VFIO_BASE + 8)) as u64;

// structs
#[repr(C)]
struct vfio_group_status {
    argsz: u32,
    flags: u32,
}

#[repr(C)]
pub(crate) struct vfio_region_info {
    pub(crate) argsz: u32,
    pub(crate) flags: u32,
    pub(crate) index: u32,
    pub(crate) cap_offset: u32,
    pub(crate) size: u64,
    pub(crate) offset: u64,
}

pub(crate) struct VfioContainer {
    fd: Arc<File>,
}

impl VfioContainer {
    pub(crate) fn new() -> Result<Self> {
        let vfio_container = Self::init()?;
        vfio_container.check()?;
        Ok(vfio_container)
    }

    fn init() -> Result<Self> {
        println!("Initializing VFIO-based NVMe communication...");
        let container = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/vfio/vfio")?;
        Ok(Self {fd: Arc::new(container)})
    }

    fn check(&self) -> Result<()> {
        let container_fd = self.fd.as_raw_fd();

        // check api version
        let api_version = get_api_version(container_fd)?;
        println!("VFIO API version: {}", api_version);
        if api_version != VFIO_API_VERSION_EXPECTED {
            bail! {"VFIO API version mismatch"};
        }

        // check expected expected extension
        if !check_extension(container_fd, VFIO_IOMMU_TYPE1V2)? {
            bail! {"VFIO TYPE1v2 IOMMU not supported"};
        }
        println!("VFIO supports TYPE1v2 IOMMU");

        //self.fd = unsafe { File::from_raw_fd(container_fd) };
        Ok(())
    }

    // group_path == "/dev/vfio/42"
    // device_str == "0000:02:00.0"
    pub(crate) fn open_device(&self, group_path: &str, device_str: &str) -> Result<File> {
        let group = OpenOptions::new().read(true).write(true).open(group_path)?;
        let group_fd = group.as_raw_fd();

        // Query VFIO group status.
        let mut group_status = vfio_group_status {
            argsz: std::mem::size_of::<vfio_group_status>() as u32,
            flags: 0,
        };
        let ret = unsafe { libc::ioctl(group_fd, VFIO_GROUP_GET_STATUS, &mut group_status) };
        if ret < 0 {
            bail! {io::Error::last_os_error()};
        }
        if group_status.flags & VFIO_GROUP_FLAGS_VIABLE == 0 {
            bail! {"VFIO group not viable"};
        }
        println!("VFIO group is viable");

        // Associate the VFIO group with the container
        let container_fd = self.fd.as_raw_fd();
        let ret = unsafe { libc::ioctl(group_fd, VFIO_GROUP_SET_CONTAINER, &container_fd) };
        if ret < 0 {
            bail! {io::Error::last_os_error()};
        }
        println!("VFIO group associated with container");

        // Set the IOMMU type
        set_iommu(container_fd, VFIO_IOMMU_TYPE1V2)?;
        println!("IOMMU set to TYPE1V2");

        let device_str = CString::new(device_str)?;
        let device_fd = unsafe { libc::ioctl(group_fd, VFIO_GROUP_GET_DEVICE_FD, device_str.as_ptr()) };
        if device_fd < 0 {
            bail! {"Failed to get device FD"};
        }
        println!("Obtained VFIO device FD: {}", device_fd);

        let device = unsafe { File::from_raw_fd(device_fd) };
        Ok(device)
    }
}

// helper fns
fn get_api_version(fd: i32) -> io::Result<u32> {
    let ret = unsafe { libc::ioctl(fd, VFIO_GET_API_VERSION_IOCTL) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret as u32)
    }
}

fn check_extension(fd: i32, extension: u32) -> io::Result<bool> {
    let ret = unsafe { libc::ioctl(fd, VFIO_CHECK_EXTENSION_IOCTL, extension) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret != 0)
    }
}

fn set_iommu(fd: i32, iommu_type: u32) -> io::Result<()> {
    let ret = unsafe { libc::ioctl(fd, VFIO_SET_IOMMU_IOCTL, iommu_type) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub(crate) fn get_region_info(device_fd: i32) -> io::Result<vfio_region_info> {
    let mut region_info = vfio_region_info {
        argsz: std::mem::size_of::<vfio_region_info>() as u32,
        flags: 0,
        index: 0, // TODO: fix hardcoded BAR0
        cap_offset: 0,
        size: 0,
        offset: 0,
    };
    let ret = unsafe { libc::ioctl(device_fd, VFIO_DEVICE_GET_REGION_INFO, &mut region_info) };
    if ret < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(region_info)
}
