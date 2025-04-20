## Projects

- [codegen/generate-pci-ids](codegen/generate-pci-ids)  
  *Generates projects/pci/src/ids.rs from structured content in projects/pci-ids*

- [examples/kv](examples/kv)  
  *Playing around with nvme*

- [examples/dump-pci](examples/dump-pci)  
  *Dumps pci device info (no capability walking yet)*

- [projects/nvme](projects/nvme)  
  *NVMe spec*

- [projects/pci](projects/pci)  
  *PCI spec*

- [projects/pci-ids](projects/pci-ids)  
  *pci.ids file parsing*

- [projects/vfio](projects/vfio)  
  *Vfio protocol*


### `cargo run -p dump-info`
```bash
Successfully parsed device at 0000:03:00.0
[examples/dump-pci/src/main.rs:15:13] pci_device = PciDevice {
    vendor_id: 6535,
    device_id: 20504,
    command: PciCommandRegister {
        _reserved_07: 0,
        parity_error_response: false,
        vga_palette_snoop: false,
        memory_write_and_invalidate_enable: false,
        special_cycles: false,
        bus_master: true,
        memory_space: true,
        io_space: false,
        _reserved_15_11: 0,
        interrupt_disable: true,
        fast_back_to_back_enable: false,
        serr_enable: false,
    },
    status: PciStatusRegister {
        fast_back_to_back_capable: false,
        _reserved_06: 0,
        _66mhz_capable: false,
        capabilities_list: true,
        interupt_status: false,
        _reserved_02_00: 0,
        deteced_parity_error: false,
        signalled_system_error: false,
        received_master_abort: false,
        received_target_abort: false,
        signalled_target_abort: false,
        devsel_timing: Fast,
        master_data_parity_error: false,
    },
    revision_id: 1,
    prog_if: 2,
    subclass: 8,
    class_code: 1,
    cache_line_size: 16,
    latency_timer: 0,
    header_type: PciHeader {
        multifunction: false,
        layout: 0,
    },
    bist: PciBIST {
        supported: false,
        start_test: false,
        _reserved_05_04: 0,
        failure_code: 0,
    },
    layout: Type0(
        PciBaseLayout {
            bar: [
                4152360964,
                0,
                0,
                0,
                0,
                0,
            ],
            cardbus_cis_pointer: 0,
            subsystem_vendor_id: 6535,
            subsystem_id: 20504,
            expansion_rom_base_address: 0,
            capabilities_pointer: 128,
            reserved1: [
                0,
                0,
                0,
            ],
            reserved2: 0,
            interrupt_line: 255,
            interrupt_pin: 1,
            min_grant: 0,
            max_latency: 0,
        },
    ),
    pci_id: MassStorageController(
        NonVolatileMemoryController(
            NVMExpress,
        ),
    ),
}
Successfully parsed device at 0000:00:03.1
[examples/dump-pci/src/main.rs:15:13] pci_device = PciDevice {
    vendor_id: 4130,
    device_id: 5285,
```
