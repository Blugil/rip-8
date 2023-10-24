use std::fs::File;
use std::io::{self, Read};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Rip8 {
    pub display: Vec<Vec<bool>>,
    pub buffer: Vec<u8>,
    pub registers: Vec<u8>,
    pub stack: Vec<u16>,
    pub sp: u16,
    pub i: u16,
    pub pc: u16,
    pub delay: u8,
    pub sound: u8,
    pub pause: bool,
    pub speed: u32,
    pub keypress: u16,
    pub keydown: Vec<bool>,
}

impl Rip8 {
    pub fn new() -> Self {
        let mut buffer = vec![0 as u8; 4096];

        let base_sprites = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..base_sprites.len() {
            buffer[0x050 + i] = base_sprites[i];
        }

        Self {
            // 2d [y][x] boolean vector
            display: vec![vec![false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            buffer,
            registers: vec![0 as u8; 16],
            stack: vec![0; 16],
            sp: 0,
            i: 0x200,
            delay: 0,
            sound: 0,
            pc: 0x200,
            pause: false,
            speed: 10,
            keypress: 0xFF,
            keydown: vec![false; 16],
        }
    }

    pub fn load_program(&mut self, path: String) -> io::Result<()> {
        // some code to ensure proper bytes

        let mut f = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::new();
        // perhaps can read directly into the buffer to save time?
        match f.read_to_end(&mut buffer) {
            Ok(_) => {
                for i in 0..buffer.len() {
                    self.buffer[0x200 + i] = buffer[i];
                }
                println!("loaded the program!");
                Ok(())
            }
            Err(e) => Err(e),
        }

        //print the first opcode of the program
    }

    // needs to report collision
    pub fn invert_pixel(&mut self, x: usize, y: usize) {
        //swap pixel values

        // handles index wrapping
        let mut x_wrap: usize = x;
        let mut y_wrap: usize = y;
        if x >= 64 {
            x_wrap = (x % 64).try_into().unwrap();
        }
        if y >= 32 {
            y_wrap = (y % 32).try_into().unwrap();
        }
        // swaps the bit at the correct coordinate
        self.display[y_wrap][x_wrap] = !self.display[y_wrap][x_wrap];
    }

    pub fn clear(&mut self) {
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                self.display[j][i] = false;
            }
        }
    }
}
