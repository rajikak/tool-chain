#[derive(Debug)]
pub struct Elf64File {
    pub header:Elf64Header,
}

#[derive(Debug)]
pub struct Elf64Header {
    // https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h#L234
    pub ident:Elf64Identifier, // magic number and other info
}

#[derive(Debug)]
pub struct Elf64Identifier {
    
}


