use std::time::Duration;

use super::rip8::Rip8;

pub struct Cpu {
    pub clock_speed: u32,
}

impl Cpu {

    pub fn emulate_cycle(&self, rip8: &mut Rip8) {
        let opcode = u16::from(rip8.buffer.get(usize::from(rip8.pc)).unwrap().to_owned()) << 8
            | u16::from(
                rip8.buffer
                    .get(usize::from(rip8.pc + 1))
                    .unwrap()
                    .to_owned(),
            );

        match opcode & 0xF000 {
            0xA000 => {
                rip8.i = opcode & 0x0FFF;
                rip8.pc += 2;
            },
            0x6000 => {
                let register = (opcode & 0x0F00) >> 8;
                let value = opcode & 0x00FF;
                rip8.registers[usize::from(register)] = value as u8;
                rip8.pc += 2;
            },
            0xD000 => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                let n = opcode & 0x000F;

                rip8.pc += 2;
            },
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        //clear the screen
                        println!("cleared the screen!");
                        rip8.clear();
                        rip8.pc += 2;
                    }
                    0x000E => {
                        //return from subroutine
                        println!("returned from subroutine!");
                        rip8.pc += 2;
                    }
                    _ => panic!("unknown opcode")
                }
            },
            _ => panic!("unknown opcode") 
        }

        if rip8.delay > 0 {
            rip8.delay -= 1;
        }

        if rip8.sound > 0 {
            println!("beep!");
            rip8.sound -= 1;
        }
    }

    #[allow(unused)]
    pub fn start(&self) {
        'clock: loop {
            // cpu clock speed
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.clock_speed));
        }
    }
}
