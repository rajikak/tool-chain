use std::fs::File;
use std::io::{Read};
use clap::Parser;

mod elf;
mod macho;
mod parser;


fn error(action:&str) -> ! {
    eprintln!("something went wrong {} that file.", action);
    std::process::exit(1)
}

#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    #[arg(short, default_value="hexdump")]
    tool:String,

    #[arg(short)]
    file:String,
}

fn main() {

    let args = Args::parse();

    println!("{}, {}", args.file, args.tool);

    let file = "assets/hello.o";
    let mut buf = Vec::new();
    let mut file = File::open(file).unwrap_or_else(|_| error("opening"));
    file.read_to_end(&mut buf).unwrap_or_else(|_| error("reading"));
    buf.reverse();

    print!("{:#x}, {:#x}, {:#x}, {:#x}, {:#x}", buf.pop().unwrap(), buf.pop().unwrap(), buf.pop().unwrap(), buf.pop().unwrap(), buf.pop().unwrap());
}
