use std::fmt::format;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Hexer {
    buf: Vec<u8>,
}

impl Hexer {
    // print a binary file in hexadecimal
    // https://github.com/torvalds/linux/blob/master/lib/hexdump.c

    pub fn new(buf: Vec<u8>) -> Self {
        Self { buf }
    }

    pub fn hex(&self, n: u8) -> String {
        let s: Vec<String> = self
            .buf
            .chunks(2)
            .map(|c| {
                if c.len() == 2 {
                    format!("{:02x}{:02x}", c[0], c[1])
                } else {
                    format!("{:02x}", c[0])
                }
            })
            .collect();
        
        s.join(" ")
    }
}
