use deku::{DekuRead, DekuWrite};
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MassStorageControllerIDEInterfaceProgIf {
    #[deku(id = 0x00)]
    ISACompatibilityModeOnlyController,
    #[deku(id = 0x05)]
    PCINativeModeOnlyController,
    #[deku(id = 0x0A)]
    ISACompatibilityModeControllerSupportsBothChannelsSwitchedToPCINativeMode,
    #[deku(id = 0x0F)]
    PCINativeModeControllerSupportsBothChannelsSwitchedToISACompatibilityMode,
    #[deku(id = 0x80)]
    ISACompatibilityModeOnlyControllerSupportsBusMastering,
    #[deku(id = 0x85)]
    PCINativeModeOnlyControllerSupportsBusMastering,
    #[deku(id = 0x8A)]
    ISACompatibilityModeControllerSupportsBothChannelsSwitchedToPCINativeModeSupportsBusMastering,
    #[deku(id = 0x8F)]
    PCINativeModeControllerSupportsBothChannelsSwitchedToISACompatibilityModeSupportsBusMastering,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MassStorageControllerATAControllerProgIf {
    #[deku(id = 0x20)]
    ADMASingleStepping,
    #[deku(id = 0x30)]
    ADMAContinuousOperation,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MassStorageControllerSATAControllerProgIf {
    #[deku(id = 0x00)]
    VendorSpecific,
    #[deku(id = 0x01)]
    AHCI10,
    #[deku(id = 0x02)]
    SerialStorageBus,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MassStorageControllerSerialAttachedSCSIControllerProgIf {
    #[deku(id = 0x01)]
    SerialStorageBus,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MassStorageControllerNonVolatileMemoryControllerProgIf {
    #[deku(id = 0x01)]
    NVMHCI,
    #[deku(id = 0x02)]
    NVMExpress,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MassStorageControllerUniversalFlashStorageControllerProgIf {
    #[deku(id = 0x00)]
    VendorSpecific,
    #[deku(id = 0x01)]
    UFSHCI,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum DisplayControllerVGACompatibleControllerProgIf {
    #[deku(id = 0x00)]
    VGAController,
    #[deku(id = 0x01)]
    _8514Controller,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum MemoryControllerCXLProgIf {
    #[deku(id = 0x00)]
    CXLMemoryDeviceVendorSpecific,
    #[deku(id = 0x10)]
    CXLMemoryDeviceCXL2X,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum BridgePCIBridgeProgIf {
    #[deku(id = 0x00)]
    NormalDecode,
    #[deku(id = 0x01)]
    SubtractiveDecode,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum BridgeRACEwayBridgeProgIf {
    #[deku(id = 0x00)]
    TransparentMode,
    #[deku(id = 0x01)]
    EndpointMode,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum BridgeSemiTransparentPCIToPCIBridgeProgIf {
    #[deku(id = 0x40)]
    PrimaryBusTowardsHostCPU,
    #[deku(id = 0x80)]
    SecondaryBusTowardsHostCPU,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum CommunicationControllerSerialControllerProgIf {
    #[deku(id = 0x00)]
    _8250,
    #[deku(id = 0x01)]
    _16450,
    #[deku(id = 0x02)]
    _16550,
    #[deku(id = 0x03)]
    _16650,
    #[deku(id = 0x04)]
    _16750,
    #[deku(id = 0x05)]
    _16850,
    #[deku(id = 0x06)]
    _16950,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum CommunicationControllerParallelControllerProgIf {
    #[deku(id = 0x00)]
    SPP,
    #[deku(id = 0x01)]
    BiDir,
    #[deku(id = 0x02)]
    ECP,
    #[deku(id = 0x03)]
    IEEE1284,
    #[deku(id = 0xFE)]
    IEEE1284Target,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum CommunicationControllerModemProgIf {
    #[deku(id = 0x00)]
    Generic,
    #[deku(id = 0x01)]
    Hayes16450,
    #[deku(id = 0x02)]
    Hayes16550,
    #[deku(id = 0x03)]
    Hayes16650,
    #[deku(id = 0x04)]
    Hayes16750,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum GenericSystemPeripheralPICProgIf {
    #[deku(id = 0x00)]
    _8259,
    #[deku(id = 0x01)]
    ISAPIC,
    #[deku(id = 0x02)]
    EISAPIC,
    #[deku(id = 0x10)]
    IOAPIC,
    #[deku(id = 0x20)]
    IOXAPIC,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum GenericSystemPeripheralDMAControllerProgIf {
    #[deku(id = 0x00)]
    _8237,
    #[deku(id = 0x01)]
    ISADMA,
    #[deku(id = 0x02)]
    EISADMA,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum GenericSystemPeripheralTimerProgIf {
    #[deku(id = 0x00)]
    _8254,
    #[deku(id = 0x01)]
    ISATimer,
    #[deku(id = 0x02)]
    EISATimers,
    #[deku(id = 0x03)]
    HPET,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum GenericSystemPeripheralRTCProgIf {
    #[deku(id = 0x00)]
    Generic,
    #[deku(id = 0x01)]
    ISARTC,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum GenericSystemPeripheralTimingCardProgIf {
    #[deku(id = 0x01)]
    TAPTimingCard,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum InputDeviceControllerGameportControllerProgIf {
    #[deku(id = 0x00)]
    Generic,
    #[deku(id = 0x10)]
    Extended,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum SerialBusControllerFireWireIEEE1394ProgIf {
    #[deku(id = 0x00)]
    Generic,
    #[deku(id = 0x10)]
    OHCI,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum SerialBusControllerUSBControllerProgIf {
    #[deku(id = 0x00)]
    UHCI,
    #[deku(id = 0x10)]
    OHCI,
    #[deku(id = 0x20)]
    EHCI,
    #[deku(id = 0x30)]
    XHCI,
    #[deku(id = 0x40)]
    USB4HostInterface,
    #[deku(id = 0x80)]
    Unspecified,
    #[deku(id = 0xFE)]
    USBDevice,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum SerialBusControllerIPMIInterfaceProgIf {
    #[deku(id = 0x00)]
    SMIC,
    #[deku(id = 0x01)]
    KCS,
    #[deku(id = 0x02)]
    BTBlockTransfer,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum UnclassifiedDeviceSubtype {
    #[deku(id = 0x00)]
    NonVGAUnclassifiedDevice,
    #[deku(id = 0x01)]
    VGACompatibleUnclassifiedDevice,
    #[deku(id = 0x05)]
    ImageCoprocessor,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum MassStorageControllerSubtype {
    #[deku(id = 0x00)]
    SCSIStorageController,
    #[deku(id = 0x01)]
    IDEInterface(#[deku(ctx = "prog_if")] MassStorageControllerIDEInterfaceProgIf),
    #[deku(id = 0x02)]
    FloppyDiskController,
    #[deku(id = 0x03)]
    IPIBusController,
    #[deku(id = 0x04)]
    RAIDBusController,
    #[deku(id = 0x05)]
    ATAController(#[deku(ctx = "prog_if")] MassStorageControllerATAControllerProgIf),
    #[deku(id = 0x06)]
    SATAController(#[deku(ctx = "prog_if")] MassStorageControllerSATAControllerProgIf),
    #[deku(id = 0x07)]
    SerialAttachedSCSIController(
        #[deku(ctx = "prog_if")] MassStorageControllerSerialAttachedSCSIControllerProgIf,
    ),
    #[deku(id = 0x08)]
    NonVolatileMemoryController(
        #[deku(ctx = "prog_if")] MassStorageControllerNonVolatileMemoryControllerProgIf,
    ),
    #[deku(id = 0x09)]
    UniversalFlashStorageController(
        #[deku(ctx = "prog_if")] MassStorageControllerUniversalFlashStorageControllerProgIf,
    ),
    #[deku(id = 0x80)]
    MassStorageController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum NetworkControllerSubtype {
    #[deku(id = 0x00)]
    EthernetController,
    #[deku(id = 0x01)]
    TokenRingNetworkController,
    #[deku(id = 0x02)]
    FDDINetworkController,
    #[deku(id = 0x03)]
    ATMNetworkController,
    #[deku(id = 0x04)]
    ISDNController,
    #[deku(id = 0x05)]
    WorldFipController,
    #[deku(id = 0x06)]
    PICMGController,
    #[deku(id = 0x07)]
    InfinibandController,
    #[deku(id = 0x08)]
    FabricController,
    #[deku(id = 0x80)]
    NetworkController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum DisplayControllerSubtype {
    #[deku(id = 0x00)]
    VGACompatibleController(
        #[deku(ctx = "prog_if")] DisplayControllerVGACompatibleControllerProgIf,
    ),
    #[deku(id = 0x01)]
    XGACompatibleController,
    #[deku(id = 0x02)]
    _3DController,
    #[deku(id = 0x80)]
    DisplayController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum MultimediaControllerSubtype {
    #[deku(id = 0x00)]
    MultimediaVideoController,
    #[deku(id = 0x01)]
    MultimediaAudioController,
    #[deku(id = 0x02)]
    ComputerTelephonyDevice,
    #[deku(id = 0x03)]
    AudioDevice,
    #[deku(id = 0x80)]
    MultimediaController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum MemoryControllerSubtype {
    #[deku(id = 0x00)]
    RAMMemory,
    #[deku(id = 0x01)]
    FLASHMemory,
    #[deku(id = 0x02)]
    CXL(#[deku(ctx = "prog_if")] MemoryControllerCXLProgIf),
    #[deku(id = 0x80)]
    MemoryController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum BridgeSubtype {
    #[deku(id = 0x00)]
    HostBridge,
    #[deku(id = 0x01)]
    ISABridge,
    #[deku(id = 0x02)]
    EISABridge,
    #[deku(id = 0x03)]
    MicroChannelBridge,
    #[deku(id = 0x04)]
    PCIBridge(#[deku(ctx = "prog_if")] BridgePCIBridgeProgIf),
    #[deku(id = 0x05)]
    PCMCIABridge,
    #[deku(id = 0x06)]
    NuBusBridge,
    #[deku(id = 0x07)]
    CardBusBridge,
    #[deku(id = 0x08)]
    RACEwayBridge(#[deku(ctx = "prog_if")] BridgeRACEwayBridgeProgIf),
    #[deku(id = 0x09)]
    SemiTransparentPCIToPCIBridge(
        #[deku(ctx = "prog_if")] BridgeSemiTransparentPCIToPCIBridgeProgIf,
    ),
    #[deku(id = 0x0A)]
    InfiniBandToPCIHostBridge,
    #[deku(id = 0x80)]
    Bridge,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum CommunicationControllerSubtype {
    #[deku(id = 0x00)]
    SerialController(#[deku(ctx = "prog_if")] CommunicationControllerSerialControllerProgIf),
    #[deku(id = 0x01)]
    ParallelController(#[deku(ctx = "prog_if")] CommunicationControllerParallelControllerProgIf),
    #[deku(id = 0x02)]
    MultiportSerialController,
    #[deku(id = 0x03)]
    Modem(#[deku(ctx = "prog_if")] CommunicationControllerModemProgIf),
    #[deku(id = 0x04)]
    GPIBController,
    #[deku(id = 0x05)]
    SmardCardController,
    #[deku(id = 0x80)]
    CommunicationController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum GenericSystemPeripheralSubtype {
    #[deku(id = 0x00)]
    PIC(#[deku(ctx = "prog_if")] GenericSystemPeripheralPICProgIf),
    #[deku(id = 0x01)]
    DMAController(#[deku(ctx = "prog_if")] GenericSystemPeripheralDMAControllerProgIf),
    #[deku(id = 0x02)]
    Timer(#[deku(ctx = "prog_if")] GenericSystemPeripheralTimerProgIf),
    #[deku(id = 0x03)]
    RTC(#[deku(ctx = "prog_if")] GenericSystemPeripheralRTCProgIf),
    #[deku(id = 0x04)]
    PCIHotPlugController,
    #[deku(id = 0x05)]
    SDHostController,
    #[deku(id = 0x06)]
    IOMMU,
    #[deku(id = 0x80)]
    SystemPeripheral,
    #[deku(id = 0x99)]
    TimingCard(#[deku(ctx = "prog_if")] GenericSystemPeripheralTimingCardProgIf),
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum InputDeviceControllerSubtype {
    #[deku(id = 0x00)]
    KeyboardController,
    #[deku(id = 0x01)]
    DigitizerPen,
    #[deku(id = 0x02)]
    MouseController,
    #[deku(id = 0x03)]
    ScannerController,
    #[deku(id = 0x04)]
    GameportController(#[deku(ctx = "prog_if")] InputDeviceControllerGameportControllerProgIf),
    #[deku(id = 0x80)]
    InputDeviceController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum DockingStationSubtype {
    #[deku(id = 0x00)]
    GenericDockingStation,
    #[deku(id = 0x80)]
    DockingStation,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum ProcessorSubtype {
    #[deku(id = 0x00)]
    _386,
    #[deku(id = 0x01)]
    _486,
    #[deku(id = 0x02)]
    Pentium,
    #[deku(id = 0x10)]
    Alpha,
    #[deku(id = 0x20)]
    PowerPC,
    #[deku(id = 0x30)]
    MIPS,
    #[deku(id = 0x40)]
    CoProcessor,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum SerialBusControllerSubtype {
    #[deku(id = 0x00)]
    FireWireIEEE1394(#[deku(ctx = "prog_if")] SerialBusControllerFireWireIEEE1394ProgIf),
    #[deku(id = 0x01)]
    ACCESSBus,
    #[deku(id = 0x02)]
    SSA,
    #[deku(id = 0x03)]
    USBController(#[deku(ctx = "prog_if")] SerialBusControllerUSBControllerProgIf),
    #[deku(id = 0x04)]
    FibreChannel,
    #[deku(id = 0x05)]
    SMBus,
    #[deku(id = 0x06)]
    InfiniBand,
    #[deku(id = 0x07)]
    IPMIInterface(#[deku(ctx = "prog_if")] SerialBusControllerIPMIInterfaceProgIf),
    #[deku(id = 0x08)]
    SERCOSInterface,
    #[deku(id = 0x09)]
    CANBUS,
    #[deku(id = 0x80)]
    SerialBusController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum WirelessControllerSubtype {
    #[deku(id = 0x00)]
    IRDAController,
    #[deku(id = 0x01)]
    ConsumerIRController,
    #[deku(id = 0x10)]
    RFController,
    #[deku(id = 0x11)]
    Bluetooth,
    #[deku(id = 0x12)]
    Broadband,
    #[deku(id = 0x20)]
    _8021aController,
    #[deku(id = 0x21)]
    _8021bController,
    #[deku(id = 0x80)]
    WirelessController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum IntelligentControllerSubtype {
    #[deku(id = 0x00)]
    I2O,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum SatelliteCommunicationsControllerSubtype {
    #[deku(id = 0x01)]
    SatelliteTVController,
    #[deku(id = 0x02)]
    SatelliteAudioCommunicationController,
    #[deku(id = 0x03)]
    SatelliteVoiceCommunicationController,
    #[deku(id = 0x04)]
    SatelliteDataCommunicationController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum EncryptionControllerSubtype {
    #[deku(id = 0x00)]
    NetworkAndComputingEncryptionDevice,
    #[deku(id = 0x10)]
    EntertainmentEncryptionDevice,
    #[deku(id = 0x80)]
    EncryptionController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum SignalProcessingControllerSubtype {
    #[deku(id = 0x00)]
    DPIOModule,
    #[deku(id = 0x01)]
    PerformanceCounters,
    #[deku(id = 0x10)]
    CommunicationSynchronizer,
    #[deku(id = 0x20)]
    SignalProcessingManagement,
    #[deku(id = 0x80)]
    SignalProcessingController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8")]
pub enum ProcessingAcceleratorsSubtype {
    #[deku(id = 0x00)]
    ProcessingAccelerators,
    #[deku(id = 0x01)]
    SNIASmartDataAcceleratorInterfaceSDXIController,
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "class_code", ctx = "class_code: u8, subclass: u8, prog_if: u8")]
pub enum PciDeviceClass {
    #[deku(id = 0x00)]
    UnclassifiedDevice(#[deku(ctx = "subclass")] UnclassifiedDeviceSubtype),
    #[deku(id = 0x01)]
    MassStorageController(#[deku(ctx = "subclass, prog_if")] MassStorageControllerSubtype),
    #[deku(id = 0x02)]
    NetworkController(#[deku(ctx = "subclass")] NetworkControllerSubtype),
    #[deku(id = 0x03)]
    DisplayController(#[deku(ctx = "subclass, prog_if")] DisplayControllerSubtype),
    #[deku(id = 0x04)]
    MultimediaController(#[deku(ctx = "subclass")] MultimediaControllerSubtype),
    #[deku(id = 0x05)]
    MemoryController(#[deku(ctx = "subclass, prog_if")] MemoryControllerSubtype),
    #[deku(id = 0x06)]
    Bridge(#[deku(ctx = "subclass, prog_if")] BridgeSubtype),
    #[deku(id = 0x07)]
    CommunicationController(#[deku(ctx = "subclass, prog_if")] CommunicationControllerSubtype),
    #[deku(id = 0x08)]
    GenericSystemPeripheral(#[deku(ctx = "subclass, prog_if")] GenericSystemPeripheralSubtype),
    #[deku(id = 0x09)]
    InputDeviceController(#[deku(ctx = "subclass, prog_if")] InputDeviceControllerSubtype),
    #[deku(id = 0x0A)]
    DockingStation(#[deku(ctx = "subclass")] DockingStationSubtype),
    #[deku(id = 0x0B)]
    Processor(#[deku(ctx = "subclass")] ProcessorSubtype),
    #[deku(id = 0x0C)]
    SerialBusController(#[deku(ctx = "subclass, prog_if")] SerialBusControllerSubtype),
    #[deku(id = 0x0D)]
    WirelessController(#[deku(ctx = "subclass")] WirelessControllerSubtype),
    #[deku(id = 0x0E)]
    IntelligentController(#[deku(ctx = "subclass")] IntelligentControllerSubtype),
    #[deku(id = 0x0F)]
    SatelliteCommunicationsController(
        #[deku(ctx = "subclass")] SatelliteCommunicationsControllerSubtype,
    ),
    #[deku(id = 0x10)]
    EncryptionController(#[deku(ctx = "subclass")] EncryptionControllerSubtype),
    #[deku(id = 0x11)]
    SignalProcessingController(#[deku(ctx = "subclass")] SignalProcessingControllerSubtype),
    #[deku(id = 0x12)]
    ProcessingAccelerators(#[deku(ctx = "subclass")] ProcessingAcceleratorsSubtype),
    #[deku(id = 0x13)]
    NonEssentialInstrumentation,
    #[deku(id = 0x40)]
    Coprocessor,
    #[deku(id = 0xFF)]
    UnassignedClass,
}
impl Default for PciDeviceClass {
    fn default() -> Self {
        PciDeviceClass::UnassignedClass
    }
}
