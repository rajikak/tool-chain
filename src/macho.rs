#[derive(Debug)]
pub struct MachoFile {
    // https://github.com/aidansteele/osx-abi-macho-file-format-reference
    pub header: MachoHeader
}

#[derive(Debug)]
pub struct MachoHeader {
    pub ident:MachoIdentifier,         
}

#[derive(Debug)]
pub struct MachoIdentifier {
    
}