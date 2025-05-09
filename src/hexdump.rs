use std::fs::File;
use std::io::Read;
use crate::error;

#[derive(Debug)]
pub struct Hex {
    file:String, 
} 

impl Hex {
    
    // print a binary file in hexadecimal 
    // https://github.com/torvalds/linux/blob/master/lib/hexdump.c
    
    pub fn new(file: String) -> Self {
        Self { file }
    }
    
    pub fn print(&self, n:u8) {
        let mut buf = Vec::new();
        let mut file = File::open(&self.file).unwrap_or_else(|_| error("opening"));
        file.read_to_end(&mut buf).unwrap_or_else(|_| error("reading"));
        buf.reverse();

        print!("{:#x}, {:#x}, {:#x}, {:#x}, {:#x}", buf.pop().unwrap(), buf.pop().unwrap(), buf.pop().unwrap(), buf.pop().unwrap(), buf.pop().unwrap());
        
    }
}