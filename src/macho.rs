use nom::number::Endianness;

pub const MAGIC: u32 = 0xfeed_face; // magic field - 32 bit https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L65
pub const CIGAM: u32 = 0xcdfa_edfe; // reverse -- little endian (need to swap bytes)

pub const MAGIC64: u32 = 0xfeed_facf; // magic field - 64 bit https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L84
pub const CIGAM64: u32 = 0xcffa_edfe; // reverse -- little endian (need to swap bytes)

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
        format!("{}:\n{}", file, self.header.str())
    }
}

#[derive(Debug)]
pub struct MachHeader {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L72
    pub magic: u32,
    pub endianness: Endianness,
}

impl MachHeader {
    pub fn parse(input: &[u8]) -> Result<MachHeader, &str> {
        let res = u32::from_str_radix(
            format!("{:x}{:x}{:x}{:x}", input[0], input[1], input[2], input[3]).as_str(),
            16,
        );
        if let Ok(MAGIC) = res {
            let magic = MAGIC;
            let endianness = Endianness::Big;
            Ok(MachHeader { magic, endianness })
        } else if let Ok(MAGIC64) = res {
            let magic = MAGIC64;
            let endianness = Endianness::Big;
            Ok(MachHeader { magic, endianness })
        } else if let Ok(CIGAM) = res {
            let magic = CIGAM;
            let endianness = Endianness::Little;
            Ok(MachHeader{magic, endianness})
        } else if let Ok(CIGAM64) = res {
            let magic = CIGAM64;
            let endianness = Endianness::Little;
            Ok(MachHeader{magic,endianness})
        } else {
            Err("not a matching matcho header")
        }
    }

    pub fn str(&self) -> String {
        let mut endianness = "";
        if self.endianness == Endianness::Little {
            endianness = "little"
        } else if self.endianness == Endianness::Big {
            endianness = "big"
        } else {
            endianness = "native"
        }
        format!("match header\nendianness: {}\nmagic: 0x{:x}", endianness, self.magic)
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
