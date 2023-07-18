mod rip8;
use rip8::render;
use rip8::rip8::Rip8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rip8 = Rip8::new();
    rip8.load_program(args[1].to_string()).unwrap();

    render::create_window(&mut rip8); // main window thread
}
