mod rip8;
use rip8::rip8::Rip8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rip = Rip8::new();
    rip.load_program(args[1].to_string()).unwrap();

}
