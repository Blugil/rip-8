mod rip8;
use rip8::render;
use rip8::rip8::Rip8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rip8 = Rip8::new();
    let rom = args[1].to_string();

    render::start_chip(&mut rip8, rom.clone()); // main window thread
}
