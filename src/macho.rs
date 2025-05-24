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
    pub cpu_type: u32,
    pub cpu_sub_type: u32,
}

fn parse(input: &[u8]) -> Result<u32, &str> {
    if input.len() != 4 {
        return Err("input should be of size 4");
    }
    match u32::from_str_radix(
        format!("{:x}{:x}{:x}{:x}", input[0], input[1], input[2], input[3]).as_str(),
        16,
    ) {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("format error {}", e);
            Err("error parsing input")
        }
    }
}

impl MachHeader {
    pub fn parse(input: &[u8]) -> Result<MachHeader, &str> {
        let magic_no = parse(&input[0..3]);
        let cpu_type =  parse(&input[4..8])?;
        let cpu_sub_type = parse(&input[9..13])?;
        if let Ok(MAGIC) = magic_no {
            let magic = MAGIC;
            let endianness = Endianness::Big;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
            })
        } else if let Ok(MAGIC64) = magic_no {
            let magic = MAGIC64;
            let endianness = Endianness::Big;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
            })
        } else if let Ok(CIGAM) = magic_no {
            let magic = CIGAM;
            let endianness = Endianness::Little;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
            })
        } else if let Ok(CIGAM64) = magic_no {
            let magic = CIGAM64;
            let endianness = Endianness::Little;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
            })
        } else {
            Err("not a matching matcho header")
        }
    }

    pub fn str(&self) -> String {
        let mut buf: Vec<String> = Vec::new();
        buf.push("match header".to_string());

        let mut endian_str: &str = "";
        if self.endianness == Endianness::Little {
            endian_str = "little";
        } else if self.endianness == Endianness::Big {
            endian_str = "big";
        } else {
            endian_str = "native";
        }

        let endian = format!("endianness: {}", endian_str);
        buf.push(endian);

        let header = format!("magic: 0x{:x}", self.magic);
        buf.push(header);

        let cpu = format!("cpu_type: {:x}, sub_type: {:x}", self.cpu_type, self.cpu_sub_type);
        buf.push(cpu);

        buf.join("\n")
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
