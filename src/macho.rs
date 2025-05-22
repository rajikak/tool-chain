use crate::Tools::Macho;
use std::fmt::format;
use std::num::ParseIntError;

pub const MAGIC: u32 = 0xfeed_face; // magic field - 32 bit https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L65
pub const CIGAM: u32 = 0xcdfa_edfe; // reverse 

pub const MAGIC64: u32 = 0xfeed_facf; // magic field - 64 bit https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L84
pub const CIGAM64: u32 = 0xcffa_edfe; // reverse

#[derive(Debug)]
pub struct MachFile {
    // https://github.com/aidansteele/osx-abi-macho-file-format-reference
    pub header: MachHeader,
    pub commands: Vec<SegmentCommand>,
    pub segments: Vec<Segment>,
}

// impl MachFile {
//     pub fn new(buf: &[u8]) -> Self {
//         Self {
//             header: MachHeader::new("".to_string()),
//             commands: Vec::new(),
//             segments: Vec::new(),
//         }
//     }
//
//     pub fn print_magic(&mut self) {
//         self.buf.reverse();
//         let mut magic: Vec<u8> = self.buf.drain(0..4).collect();
//
//         let str = format!("{:02x}{:02x}{:02x}{:02x}", magic.pop().unwrap(), magic.pop().unwrap(), magic.pop().unwrap(), magic.pop().unwrap());
//         println!("magic: 0x{}", str);
//     }
// }

#[derive(Debug)]
pub struct MachHeader {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L72
    pub magic: u32,
}

impl MachHeader {
    pub fn parse(input: &[u8]) -> Result<MachHeader, &str> {
        let res = u32::from_str_radix(
            format!("{:x}{:x}{:x}{:x}", input[0], input[1], input[2], input[3]).as_str(),
            16,
        );
        if let Ok(MAGIC) = res {
            let magic = MAGIC;
            Ok(MachHeader { magic })
        } else if let Ok(MAGIC64) = res {
            let magic = MAGIC64;
            Ok(MachHeader { magic })
        } else {
            Err("not a matching matcho header")
        }
    }
}

#[derive(Debug)]
pub struct SegmentCommand {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L355
}

#[derive(Debug)]
pub struct Segment {
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {}

#[derive(Debug)]
pub struct MachoIdentifier {}

#[derive(Debug)]
pub struct MachoCommand {}
