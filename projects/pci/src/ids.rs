use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum NonVolatileMemoryProgIf {
    #[deku(id = 0x01)] NVMHCI,
    #[deku(id = 0x02)] NVMExpress,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum MassStorageSubtype {
    #[deku(id = 0x00)] Scsi,
    #[deku(id = 0x01)] Ide,
    #[deku(id = 0x08)] NonVolatileMemory(#[deku(ctx = "prog_if")] NonVolatileMemoryProgIf),
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(id = "class_code", ctx = "class_code: u8, subclass: u8, prog_if: u8")]
pub enum PciDeviceClass {
    #[deku(id = 0x00)] UnclassifiedDevice,
    #[deku(id = 0x01)] MassStorageController(#[deku(ctx = "subclass, prog_if")] MassStorageSubtype),
    #[deku(id = 0x02)] NetworkController,
    #[deku(id = 0x03)] DisplayController,
    #[deku(id = 0x04)] MultimediaController,
    #[deku(id = 0x05)] MemoryController,
    #[deku(id = 0x06)] Bridge,
    #[deku(id = 0x07)] CommunicationController,
    #[deku(id = 0x08)] GenericSystemPeripheral,
    #[deku(id = 0x09)] InputDeviceController,
    #[deku(id = 0x0A)] DockingStation,
    #[deku(id = 0x0B)] Process,
    #[deku(id = 0x0C)] SerialBusController,
    #[deku(id = 0x0D)] WirelessController,
    #[deku(id = 0x0E)] IntelligentController,
    #[deku(id = 0x0F)] SatelliteCommunicationsController,
    #[deku(id = 0x10)] EncryptionController,
    #[deku(id = 0x11)] SignalProcessingController,
    #[deku(id = 0x12)] ProcessingAccelerators,
    #[deku(id = 0x13)] NonEssentialInstrumentation,
    #[deku(id = 0x40)] Coprocessor,
    #[default]
    #[deku(id = 0xFF)] UnassignedClass,
}
