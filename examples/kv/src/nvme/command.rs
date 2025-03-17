use deku::prelude::*;
use anyhow::{bail, Result};
use super::NvmeController;


#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
enum Opcode {
    #[deku(id = 0x06)]
    Identify,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 2, id_type = "u8")]
enum PrpOrSGLDataTransfer {
    #[deku(id = 0b00)]
    PRP,
    #[deku(id = 0b01)]
    SGLByteAligned,
    #[deku(id = 0b10)]
    SGLQwordAligned,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 2, id_type = "u8")]
enum FusedOperation {
    #[deku(id = 0b00)]
    Normal,
    #[deku(id = 0b01)]
    FusedFirstCommand,
    #[deku(id = 0b10)]
    FusedSecondCommand,
}

#[derive(Debug, DekuRead, DekuWrite)]
struct CommandDword0 {
    #[deku(endian = "big")]
    cid: u16,
    psdt: PrpOrSGLDataTransfer,
    #[deku(bits = 4)]
    _reserved_13_10: u8,
    fuse: FusedOperation,
    opc: Opcode,
}

#[derive(Debug, DekuWrite, DekuRead)]
#[deku(endian = "big", id_type = "u128")]
enum DataPointer {
    //TODO PRETTY SURE THESE IDS ARE WRONG

    #[deku(id = 0)]
    PRP {
        prp2: u64,
        prp1: u64,
    },
    #[deku(id = 1)]
    SGL {
        sgl1: u128,
    },
}

#[derive(Debug, DekuWrite, DekuRead)]
pub(crate) struct Command {
    #[deku(endian = "big")]
    cdw15: u32,
    #[deku(endian = "big")]
    cdw14: u32,
    #[deku(endian = "big")]
    cdw13: u32,
    #[deku(endian = "big")]
    cdw12: u32,
    #[deku(endian = "big")]
    cdw11: u32, // NDM with "vendor specific"
    #[deku(endian = "big")]
    cdw10: u32, // NDT with "vendor specific"
    dptr: DataPointer, // broken because i dont know deku enough to reference cdw0 below
    #[deku(endian = "big")]
    mptr:  u64,
    _reserved_15_08: u8,
    #[deku(endian = "big")]
    nsid:  u32,
    cdw0: CommandDword0,
}

impl Command {
    pub const SIZE: usize = 64;
}

impl From<Command> for super::NvmeCommand {
    fn from(cmd: Command) -> Self {
        let bytes = cmd.to_bytes().expect("Serialization failed");
        assert!(bytes.len() == Command::SIZE);

        let mut array = [0u8; Command::SIZE];
        array.copy_from_slice(&bytes[..Command::SIZE]);
        array
    }
}

#[derive(Debug, DekuWrite, DekuRead)]
pub(crate) struct CompletionQueueEntryDW3 {
    #[deku(endian = "big", bits = 15)]
    sf: u16, // section 4.6.1
    #[deku(bits = 1)]
    p: bool,
    #[deku(endian = "big")]
    cid: u16,
}

#[derive(Debug, DekuWrite, DekuRead)]
pub(crate) struct CompletionQueueEntryDW2 {
    #[deku(endian = "big")]
    sqid: u16,
    #[deku(endian = "big")]
    sqhd: u16,
}

#[derive(Debug, DekuWrite, DekuRead)]
pub(crate) struct Completion {
    #[deku(endian = "big")]
    _dw0_command_specific: u32,
    #[deku(endian = "big")]
    _dw1_reserved: u32,
    dw2: CompletionQueueEntryDW2,
    dw3: CompletionQueueEntryDW3,
}


impl Completion {
    pub const SIZE: usize = 16;
}

impl From<Completion> for super::NvmeCompletion {
    fn from(cmd: Completion) -> Self {
        let bytes = cmd.to_bytes().expect("Serialization failed");
        assert!(bytes.len() == Completion::SIZE);

        let mut array = [0u8; Completion::SIZE];
        array.copy_from_slice(&bytes[..Completion::SIZE]);
        array
    }
}
