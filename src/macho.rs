#[derive(Debug)]
pub struct MachoFile {
    // https://github.com/aidansteele/osx-abi-macho-file-format-reference
    pub header: MachoHeader,
    pub commands:Vec<SegmentCommand>,
    pub segments:Vec<Segment>,
}

#[derive(Debug)]
pub struct MachoHeader {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L54
    pub magic:u32,
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
pub struct Section {

}

#[derive(Debug)]
pub struct MachoIdentifier {

}

#[derive(Debug)]
pub struct MachoCommand {
    
}

