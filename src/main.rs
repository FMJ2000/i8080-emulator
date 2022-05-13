use std::env;
use std::fs;

fn main() {
    let mut args: env::Args = env::args();
    args.next();
    if let Some(filename) = args.next() {
        let mut buffer: Vec<u8> = fs::read(filename).unwrap();
        let mut ram: Vec<u8> = vec![0; 0x2000];
        buffer.append(&mut ram);
        invaders::emulate::emulate(buffer);
    }
    //invaders::disassemble::hexdump(args.next());
}
