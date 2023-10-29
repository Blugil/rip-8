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
            speed: 10,
            keypress: 0xFF,
            keydown: vec![false; 16],
        }
    }

    pub fn load_program(&mut self, path: String) -> io::Result<()> {
        let mut f = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::new();
        match f.read_to_end(&mut buffer) {
            Ok(_) => {
                for i in 0..buffer.len() {
                    self.buffer[0x200 + i] = buffer[i];
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn clear_display(&mut self) {
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                self.display[j][i] = false;
            }
        }
    }
}
