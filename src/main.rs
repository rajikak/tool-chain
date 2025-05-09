use std::fs::File;
use std::io::{Read};
use clap::Parser;
use crate::hexdump::Hex;

mod elf;
mod macho;
mod parser;
mod hexdump;

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
    
    if args.tool.trim() == "hexdump" {
        let hex = Hex::new(args.file);
        hex.print(10);
    } else {
        println!("not a supported tool, should be one of: hexdump, ldd, strings, strip.");
        return;
    }
}
