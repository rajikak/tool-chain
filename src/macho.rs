
pub const MAGIC:u32 = 0xfeed_face; // magic field - 32 bit https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L65
pub const CIGAM:u32 = 0xcdfa_edfe; // reverse 

pub const MAGIC64:u32 = 0xfeed_facf; // magic field - 64 bit https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L84
pub const CIGAM64:u32 = 0xcffa_edfe; // reverse

#[derive(Debug)]
pub struct MachoFile {
    buf: Vec<u8>,
    // https://github.com/aidansteele/osx-abi-macho-file-format-reference
    pub header: MachoHeader,
    pub commands: Vec<SegmentCommand>,
    pub segments: Vec<Segment>,
}

impl MachoFile {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf,
            header: MachoHeader::new("".to_string()),
            commands: Vec::new(),
            segments: Vec::new(),
        }
    }

    pub fn print_magic(&mut self) {
        self.buf.reverse();
        let mut magic: Vec<u8> = self.buf.drain(0..4).collect();

        let str = format!("{:02x}{:02x}{:02x}{:02x}", magic.pop().unwrap(), magic.pop().unwrap(), magic.pop().unwrap(), magic.pop().unwrap());
        println!("magic: 0x{}", str);
    }
}

#[derive(Debug)]
pub struct MachoHeader {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L72
    pub magic: String,
}

impl MachoHeader {
    pub fn new(magic: String) -> Self {
        Self { magic }
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
