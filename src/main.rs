use std::env;

fn main() {
    let mut args: env::Args = env::args();
    args.next();
    invaders::disassemble::hexdump(args.next());
}
