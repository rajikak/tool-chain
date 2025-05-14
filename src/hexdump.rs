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

    pub fn hex(&self, n: usize) -> String {
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
        
        let mut chunks = Vec::new();
        for chunk in s.chunks(2) {
            chunks.push(chunk.to_vec());
        }
        
        let mut row = Vec::new();
        for chunk in chunks {
            row.push(chunk.join(""));
        }
        
        let mut res = Vec::new();
        for chunk in row.chunks(n) {
            res.push(chunk.join(" "));
        }
        
        res.join("\n")
    }
}
