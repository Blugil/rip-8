use std::time::Duration;

pub struct cpu {
    pub clock_speed: u32,
}

impl cpu {
    pub fn new(&mut self, clock_speed: u32) {
        self.clock_speed = clock_speed; 
    }

    pub fn start(&self) {
        'clock: loop {
            // cpu clock speed 
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.clock_speed));
        }
    }
}
