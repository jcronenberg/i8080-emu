use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;

mod disassembler;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
        return;
    }

    let mut buffer = Vec::new();
    match read_file_to_buf(&args[2], &mut buffer) {
        Ok(()) => println!("Successfully loaded file: {}", args[2]),
        Err(i) => {
            println!("Error loading file '{}': {}", args[2], i);
            return
        }
    }

    if args[1] == "hexdump" {
        disassembler::hexdump(buffer);
    }
    else if args[1] == "disassemble" {
        //for (i, v) in buffer.iter().enumerate() {
        //    disassemble8080Op(buffer);
        let length = buffer.len();
        let mut i:usize = 0;
        while i < length {
            i += disassembler::disassemble8080_op(&buffer, i);
        }
    }
    else {
        println!("Unknown command!\n");
        usage();
    }
}

fn read_file_to_buf(file: &str, buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut f = File::open(file)?;

    f.read_to_end(buffer)?;

    Ok(())
}

fn usage() {
    println!("USAGE: i8080-emu <command> <file>\n");
    println!("COMMANDS:");
    println!("disassemble   disassemble file and output to stdout");
    println!("hexdump       hexdump file and output to stdout");
}
