extern crate rand;
use super::rip8::Rip8;
use rand::Rng;
use std::time::Duration;

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

        //println!("opcode: {:#04x}", opcode);

        let reg_x = (opcode & 0x0F00) >> 8;
        let reg_y = (opcode & 0x00F0) >> 4;
        let reg_x_value = rip8.registers[usize::from(reg_x)];
        let reg_y_value = rip8.registers[usize::from(reg_y)];

        match opcode & 0xF000 {
            //load index register with immediate value
            0x0000 => {
                // matches the 0x00EE and 0x000E opcodes
                match opcode & 0x000F {
                    0x0000 => {
                        //clear the screen
                        rip8.clear();
                        rip8.pc += 2;
                        println!("cleared the screen!");
                    }
                    0x000E => {
                        //return from subroutine
                        rip8.pc = rip8.stack.pop().unwrap() + 2;
                        rip8.sp -= 1;
                    }
                    _ => panic!("unknown opcode"),
                }
            }
            0x1000 => {
                let address = opcode & 0x0FFF;
                rip8.pc = address;
            }
            0x2000 => {
                let address: u16 = opcode & 0x0FFF;
                rip8.sp += 1;
                rip8.stack.push(rip8.pc);
                rip8.pc = address;
            }
            0x3000 => {
                let register = (opcode & 0x0F00) >> 8;
                let value = opcode & 0x00FF;
                //skip next instruction if value is equal to value stored in register
                if rip8.registers[usize::from(register)] == value as u8 {
                    rip8.pc += 2;
                }
                rip8.pc += 2;
            }
            0x4000 => {
                let value = opcode & 0x00FF;
                //skip next instruction if the value is not equal to the value stored in the
                //register
                if rip8.registers[usize::from(reg_x)] != value as u8 {
                    rip8.pc += 2;
                }
                rip8.pc += 2;
            }
            0x5000 => {
                //skip next instruction if the values are the same in Vx and Vy
                if reg_x_value == reg_y_value {
                    rip8.pc += 2;
                }
                rip8.pc += 2;
            }
            0x6000 => {
                let value = opcode & 0x00FF;
                rip8.registers[usize::from(reg_x)] = value as u8;
                rip8.pc += 2;
            }
            0x7000 => {
                let value = opcode & 0x00FF;
                rip8.registers[usize::from(reg_x)] = reg_x_value.wrapping_add(value as u8);
                rip8.pc += 2;
            }
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        rip8.registers[usize::from(reg_x)] = reg_y_value;
                        rip8.pc += 2;
                    }
                    0x0001 => {
                        rip8.registers[usize::from(reg_x)] = reg_x_value | reg_y_value;
                        rip8.pc += 2;
                    }
                    0x0002 => {
                        rip8.registers[usize::from(reg_x)] = reg_x_value & reg_y_value;
                        rip8.pc += 2;
                    }
                    0x0003 => {
                        rip8.registers[usize::from(reg_x)] = reg_x_value ^ reg_y_value;
                        rip8.pc += 2;
                    }
                    0x0004 => {
                        //
                        //checks to see if the
                        let output: u16 = reg_x_value as u16 + reg_y_value as u16;
                        //some bit math to store the carry of an aaddition in Vf
                        rip8.registers[usize::from(reg_x)] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = ((output >> 8) & 0x01) as u8;

                        rip8.pc += 2;
                    }
                    //maybe wrong implementation of negatives?
                    0x0005 => {
                        //skip next instruction if the values are the same in Vx and Vy
                        //
                        //checks to see if the
                        let output;

                        if reg_x_value > reg_y_value {
                            rip8.registers[0xF as usize] = 1;
                        } else {
                            rip8.registers[0xF as usize] = 0;
                        }

                        output = reg_x_value.overflowing_sub(reg_y_value).0;
                        rip8.registers[usize::from(reg_x)] = (output & 0xFF) as u8;
                        rip8.pc += 2;
                    }
                    0x0006 => {
                        if reg_x_value & 0x1 == 1 {
                            rip8.registers[0xF as usize] = 1;
                        } else {
                            rip8.registers[0xF as usize] = 0;
                        }
                        rip8.registers[usize::from(reg_x)] = reg_x_value / 2;

                        rip8.pc += 2;
                    }
                    0x0007 => {
                        let output;

                        if reg_y_value > reg_x_value {
                            rip8.registers[0xF as usize] = 1;
                        } else {
                            rip8.registers[0xF as usize] = 0;
                        }

                        output = reg_y_value.overflowing_sub(reg_x_value).0;
                        rip8.registers[usize::from(reg_x)] = (output & 0xFF) as u8;

                        rip8.pc += 2;
                    }
                    0x000E => {
                        if reg_x_value & 0x1 == 1 {
                            rip8.registers[0xF as usize] = 1;
                        } else {
                            rip8.registers[0xF as usize] = 0;
                        }
                        rip8.registers[usize::from(reg_x)] = reg_x_value.overflowing_mul(2).0;

                        rip8.pc += 2;
                    }
                    _ => panic!("unknown upcode"),
                }
            }
            0x9000 => {
                //skip next instruction if the values are the same in Vx and Vy
                if reg_x_value != reg_y_value {
                    rip8.pc += 2;
                }
                rip8.pc += 2;
            }
            0xA000 => {
                rip8.i = opcode & 0x0FFF;
                rip8.pc += 2;
            }
            0xB000 => {
                let value = opcode & 0x0FFF;
                rip8.pc = value + rip8.registers[0x0] as u16;
                rip8.pc += 2;
            }
            0xC000 => {
                let value = (opcode & 0x00FF) as u8;
                let rng: u8 = rand::thread_rng().gen_range(0..=255);
                rip8.registers[usize::from(reg_x)] = rng & value;
                rip8.pc += 2;
            }
            //load register with immediate value
            //draw sprite to screen
            0xD000 => {
                let n = (opcode & 0x000F) as u8;

                for mem_offset in 0..n {
                    //integer value for sprite byte stored in memory
                    let sprite = rip8.buffer[(rip8.i + mem_offset as u16) as usize];
                    for sprite_offset in 0..8 {
                        if (sprite >> sprite_offset) & 1 == 1 {
                            rip8.invert_pixel(
                                (reg_x_value + 8 - sprite_offset) as usize,
                                (reg_y_value + mem_offset) as usize,
                            );
                        }
                    }
                }

                rip8.pc += 2;
            }
            //keyboard related opcodes
            0xE000 => {
                match opcode & 0x000F {
                    0x000E => {
                        println!("beep!");
                    }
                    0x0001 => {
                        println!("beep!");
                    }
                    _ => panic!("unknown opcode"),
                }
                rip8.pc += 2;
            }

            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        println!("beep!");
                    }
                    0x000A => {
                        println!("beep!");
                    }
                    0x0015 => {
                        println!("beep!");
                    }
                    0x0018 => {
                        println!("beep!");
                    }
                    0x001E => {
                        rip8.i += reg_x_value as u16;
                    }
                    0x0029 => {
                        println!("beep!");
                    }
                    0x0033 => {
                        // really crude implementation without using the shift-add-three algo that
                        // i will implement in the future
                        let hundreds = (reg_x_value / 100) as u8;
                        // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                        let tens = ((reg_x_value / 10) % 10) as u8;
                        // Fetch the ones digit by tossing the hundreds and the tens
                        let ones = (reg_x_value % 10) as u8;

                        rip8.buffer[rip8.i as usize] = hundreds;
                        rip8.buffer[(rip8.i + 1) as usize] = tens;
                        rip8.buffer[(rip8.i + 2) as usize] = ones;
                    }
                    // reading and loading register values from memory
                    0x0055 => {
                        for x in 0..=reg_x {
                            rip8.buffer[rip8.i as usize] = rip8.registers[x as usize];
                            rip8.i += 1;
                        }
                    }
                    0x0065 => {
                        for x in 0..=reg_x {
                            rip8.registers[x as usize] = rip8.buffer[rip8.i as usize];
                            rip8.i += 1;
                        }
                    }
                    _ => panic!("unknown opcode"),
                }
                rip8.pc += 2;
            }

            _ => panic!("unknown opcode"),
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
