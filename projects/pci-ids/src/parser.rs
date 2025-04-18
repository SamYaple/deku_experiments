use crate::{Class, Device, PciIds, ProgIf, SubClass, Subsystem, Vendor};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{line_ending, not_line_ending, space1};
use nom::combinator::opt;
use nom::error::Error;
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

impl PciIds {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        (many0(Vendor::parse), many0(Class::parse))
            .map(|(vendors, classes)| Self { classes, vendors })
    }
}

impl Class {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let (input, _) = scrub_input.parse(input)?;
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        preceded(
            (tag("C"), space1),
            (
                terminated(take_u8_from_hex, space1),
                take_rest_of_line,
                many0(SubClass::parse),
            ),
        )
        .map(|(id, name, subclasses)| Self {
            id,
            name: name.to_string(),
            subclasses,
        })
    }
}

impl SubClass {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let (input, _) = scrub_input.parse(input)?;
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        preceded(
            tag("\t"),
            (
                terminated(take_u8_from_hex, space1),
                take_rest_of_line,
                many0(ProgIf::parse),
            ),
        )
        .map(|(id, name, prog_ifs)| Self { id, name: name.to_string(), prog_ifs })
    }
}

impl ProgIf {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let (input, _) = scrub_input.parse(input)?;
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        preceded(
            tag("\t\t"),
            (terminated(take_u8_from_hex, space1), take_rest_of_line),
        )
        .map(|(id, name)| Self { id, name: name.to_string() })
    }
}

impl Vendor {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let (input, _) = scrub_input.parse(input)?;
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        (
            terminated(take_u16_from_hex, space1),
            take_rest_of_line,
            many0(Device::parse),
        )
            .map(|(id, name, devices)| Self { id, name: name.to_string(), devices })
    }
}

impl Device {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let (input, _) = scrub_input.parse(input)?;
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        preceded(
            tag("\t"),
            (
                terminated(take_u16_from_hex, space1),
                take_rest_of_line,
                many0(Subsystem::parse),
            ),
        )
        .map(|(id, name, subsystems)| Self {
            id,
            name: name.to_string(),
            subsystems,
        })
    }
}

impl Subsystem {
    pub fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let (input, _) = scrub_input.parse(input)?;
        Self::parser().parse(input)
    }

    pub fn parser<'i>() -> impl Parser<&'i str, Output = Self, Error = Error<&'i str>> {
        preceded(
            tag("\t\t"),
            (
                terminated(take_u16_from_hex, space1),
                terminated(take_u16_from_hex, space1),
                take_rest_of_line,
            ),
        )
        .map(|(subvendor_id, subdevice_id, name)| Self {
            subvendor_id,
            subdevice_id,
            name: name.to_string(),
        })
    }
}

fn scrub_input(input: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((take_comment, line_ending))).parse(input)
}

fn take_comment(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), take_rest_of_line).parse(input)
}

fn take_rest_of_line(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, opt(line_ending)).parse(input)
}

fn take_u8_from_hex(input: &str) -> IResult<&str, u8> {
    take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit())
        .map_res(|hex: &str| u8::from_str_radix(hex, 16))
        .parse(input)
}

fn take_u16_from_hex(input: &str) -> IResult<&str, u16> {
    take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit())
        .map_res(|hex: &str| u16::from_str_radix(hex, 16))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_vendor_device_and_subsystems() {
        let input = "0e11  Compaq Computer Corporation\n\ta0f0  Advanced System Management Controller\n\t\t0e11 b0f3  ProLiant DL360\n\ta0f3  Triflex PCI to ISA Bridge\n\ta0f7  PCI Hotplug Controller\n\t\t8086 002a  PCI Hotplug Controller A\n\t\t8086 002b  PCI Hotplug Controller B";

        let (input, v) = Vendor::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(v.id, 0x0e11);
        assert_eq!(v.name, "Compaq Computer Corporation".to_string());
        assert_eq!(
            v.devices,
            vec![
                Device {
                    id: 0xa0f0,
                    name: "Advanced System Management Controller".to_string(),
                    subsystems: vec![Subsystem {
                        subvendor_id: 0x0e11,
                        subdevice_id: 0xb0f3,
                        name: "ProLiant DL360".to_string(),
                    },],
                },
                Device {
                    id: 0xa0f3,
                    name: "Triflex PCI to ISA Bridge".to_string(),
                    subsystems: vec![],
                },
                Device {
                    id: 0xa0f7,
                    name: "PCI Hotplug Controller".to_string(),
                    subsystems: vec![
                        Subsystem {
                            subvendor_id: 0x8086,
                            subdevice_id: 0x002a,
                            name: "PCI Hotplug Controller A".to_string(),
                        },
                        Subsystem {
                            subvendor_id: 0x8086,
                            subdevice_id: 0x002b,
                            name: "PCI Hotplug Controller B".to_string(),
                        },
                    ],
                },
            ],
        );
    }

    #[test]
    fn test_vendor() {
        let input = "01de  Oxide Computer Company";
        let (input, v) = Vendor::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(v.id, 0x01de);
        assert_eq!(v.name, "Oxide Computer Company".to_string());
        assert_eq!(v.devices, vec![]);
    }

    #[test]
    fn test_device() {
        let input = "\t0002  Propolis PCI-PCI Bridge";
        let (input, d) = Device::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(d.id, 0x0002);
        assert_eq!(d.name, "Propolis PCI-PCI Bridge".to_string());
        assert_eq!(d.subsystems, vec![]);
    }

    #[test]
    fn test_device_with_subsystems() {
        let input = "\t0b60  NVMe DC SSD [Sentinel Rock Plus controller]\n\t\t025e 8008  NVMe DC SSD U.2 15mm [D7-P5510]\n\t\t025e 8208  NVMe DC SSD U.2 15mm [D7-P5810]";
        let (input, d) = Device::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(d.id, 0x0B60);
        assert_eq!(d.name, "NVMe DC SSD [Sentinel Rock Plus controller]".to_string());
        assert_eq!(
            d.subsystems,
            vec![
                Subsystem {
                    subvendor_id: 0x025e,
                    subdevice_id: 0x8008,
                    name: "NVMe DC SSD U.2 15mm [D7-P5510]".to_string(),
                },
                Subsystem {
                    subvendor_id: 0x025e,
                    subdevice_id: 0x8208,
                    name: "NVMe DC SSD U.2 15mm [D7-P5810]".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_subdevice() {
        let input = "\t\t1028 04da  Vostro 3750";
        let (input, sd) = Subsystem::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(sd.subvendor_id, 0x1028);
        assert_eq!(sd.subdevice_id, 0x04da);
        assert_eq!(sd.name, "Vostro 3750".to_string());
    }

    #[test]
    fn test_full_class_subclass_and_prog_ifs() {
        let input = "C 03  Display controller\n\t00  VGA compatible controller\n\t\t00  VGA controller\n\t\t01  8514 controller\n\t01  XGA compatible controller\n\t02  3D controller\n\t80  Display controller";

        let (input, class) = Class::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(class.id, 0x03);
        assert_eq!(class.name, "Display controller".to_string());
        assert_eq!(
            class.subclasses,
            vec![
                SubClass {
                    id: 0x00,
                    name: "VGA compatible controller".to_string(),
                    prog_ifs: vec![
                        ProgIf {
                            id: 0x00,
                            name: "VGA controller".to_string(),
                        },
                        ProgIf {
                            id: 0x01,
                            name: "8514 controller".to_string(),
                        },
                    ]
                },
                SubClass {
                    id: 0x01,
                    name: "XGA compatible controller".to_string(),
                    prog_ifs: vec![],
                },
                SubClass {
                    id: 0x02,
                    name: "3D controller".to_string(),
                    prog_ifs: vec![],
                },
                SubClass {
                    id: 0x80,
                    name: "Display controller".to_string(),
                    prog_ifs: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_subclass_with_prog_if() {
        let input = "\t04  Gameport controller\n\t\t00  Generic\n\t\t10  Extended";

        let (input, subclass) = SubClass::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(subclass.id, 0x04);
        assert_eq!(subclass.name, "Gameport controller".to_string());
        assert_eq!(
            subclass.prog_ifs,
            vec![
                ProgIf {
                    id: 0x00,
                    name: "Generic".to_string(),
                },
                ProgIf {
                    id: 0x10,
                    name: "Extended".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_subclass() {
        let input = "\t02  FDDI network controller";

        let (input, subclass) = SubClass::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(subclass.id, 0x02);
        assert_eq!(subclass.name, "FDDI network controller".to_string());
        assert_eq!(subclass.prog_ifs, vec![]);
    }

    #[test]
    fn test_prog_if() {
        let input = "\t\t10  CXL Memory Device (CXL 2.x)";

        let (input, prog_if) = ProgIf::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(prog_if.id, 0x10);
        assert_eq!(prog_if.name, "CXL Memory Device (CXL 2.x)".to_string());
    }

    #[test]
    fn test_take_rest_of_line() {
        assert_eq!(
            take_rest_of_line(" some text matches \n but not this text \n"),
            Ok((" but not this text \n", " some text matches "))
        );
    }

    #[test]
    fn test_take_u8_from_hex() {
        assert_eq!(take_u8_from_hex("FF"), Ok(("", 255u8)));
        assert_eq!(take_u8_from_hex("ff"), Ok(("", 255u8)));
        assert_eq!(take_u8_from_hex("00"), Ok(("", 0u8)));
        assert_eq!(take_u8_from_hex("01"), Ok(("", 1u8)));
        assert_eq!(take_u8_from_hex("Ab"), Ok(("", 171u8)));
    }

    #[test]
    fn test_take_u8_from_hex_bad_length() {
        assert!(take_u8_from_hex("0").is_err());
        assert!(take_u8_from_hex("1").is_err());
        assert!(take_u8_from_hex("b").is_err());
        assert!(take_u8_from_hex("d").is_err());
    }

    #[test]
    fn test_take_u16_from_hex() {
        assert_eq!(take_u16_from_hex("FFFF"), Ok(("", 65535u16)));
        assert_eq!(take_u16_from_hex("ffff"), Ok(("", 65535u16)));
        assert_eq!(take_u16_from_hex("0000"), Ok(("", 0u16)));
        assert_eq!(take_u16_from_hex("0123"), Ok(("", 291u16)));
        assert_eq!(take_u16_from_hex("AbCd"), Ok(("", 43981u16)));
    }

    #[test]
    fn test_take_u16_from_hex_bad_length() {
        assert!(take_u16_from_hex("0").is_err());
        assert!(take_u16_from_hex("12").is_err());
        assert!(take_u16_from_hex("bad").is_err());
        assert!(take_u16_from_hex("dEa").is_err());
    }
}
