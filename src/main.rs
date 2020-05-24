use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("invaders/invaders")?;

    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    for (i, v) in buffer.iter().enumerate() {
        if i % 16 == 0 {
            print!("{:04x} ", i);
        }
        print!("{:02x} ", v);
        if i % 16 == 15 {
            print!("\n");
        }
    }

    Ok(())
}
