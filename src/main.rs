use std::fs::File;
use std::io::{BufReader, Read};

mod elf;
mod macho;
mod parser;


fn error(action:&str) -> ! {
    eprintln!("something went wrong {} that file.", action);
    std::process::exit(1)
}
fn main() {
    let file = "assets/hello.o";
    let mut buf = Vec::new();
    let mut file = File::open(file).unwrap_or_else(|_| error("opening"));
    file.read_to_end(&mut buf).unwrap_or_else(|_| error("reading"));
    
    let magic1 = buf.pop();
    let magic2 = buf.pop();
    let magic3 = buf.pop();
    let magic4 = buf.pop();
    let magic5 = buf.pop();
    print!("{:#x}, {:#x}, {:#x}, {:#x}, {:#x}", magic1.unwrap(), magic2.unwrap(), magic3.unwrap(), magic4.unwrap(), magic5.unwrap());
}
