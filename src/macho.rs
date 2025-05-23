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

impl MachFile {
    pub fn parse(input: &[u8]) -> Result<MachFile, &str> {
        let commands = Vec::new();
        let segments = Vec::new();

        match MachHeader::parse(input) {
            Ok(header) => Ok(MachFile {
                header,
                commands,
                segments,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn str(&self, file: &str) -> String {
        format!("{}", file)
    }
}

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

    pub fn str(&self) -> String {
        format!("magic: {:x}", self.magic)
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
