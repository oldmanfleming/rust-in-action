use std::{env, fs::File, io::prelude::*};

const BYTES_PER_LINE: usize = 16;

fn main() {
    let fname = env::args().nth(1).expect("usage: fview FILENAME");

    let mut f = File::open(fname).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    while f.read_exact(&mut buffer).is_ok() {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!("");
        pos += BYTES_PER_LINE;
    }
}
