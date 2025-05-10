use crate::hexdump::{Hexer};
use clap::Parser;
use std::fs::File;
use std::io::Read;

mod elf;
mod hexdump;
mod macho;

#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    #[arg(short, default_value = "hexdump")]
    tool: String,

    #[arg(short)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut buf = Vec::new();
    match File::open(&args.file) {
        Ok(mut file) => match file.read_to_end(&mut buf) {
            Ok(_) => buf.reverse(),
            Err(e2) => {
                eprintln!("error reading file content: {}", e2);
                std::process::exit(1)
            }
        },
        Err(e) => {
            eprintln!("error opening file: {}", e);
            std::process::exit(1)
        }
    }

    if args.tool.trim() == "hexdump" {
        let hex = Hexer::new(buf);
        let s = hex.hex();
        println!("{}", s)
    } else {
        println!("not a supported tool, should be one of: hexdump, ldd, strings, strip.");
        return;
    }
}
