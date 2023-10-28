extern crate rand;
use super::rip8::Rip8;
use rand::Rng;

pub struct Cpu {
    pub clock_speed: u32,
    pub timer_interval: u32,
    pub delay_state: u8,
    pub sound_state: u8,
    pub halted: bool,
}

impl Cpu {
    pub fn emulate_cycle(&mut self, rip8: &mut Rip8) {
        let opcode = u16::from(rip8.buffer.get(usize::from(rip8.pc)).unwrap().to_owned()) << 8
            | u16::from(
                rip8.buffer
                    .get(usize::from(rip8.pc + 1))
                    .unwrap()
                    .to_owned(),
            );

        let reg_x_index = (opcode & 0x0F00) >> 8;
        let reg_y_index = (opcode & 0x00F0) >> 4;
        let reg_x_value = rip8.registers[usize::from(reg_x_index)];
        let reg_y_value = rip8.registers[usize::from(reg_y_index)];

        match opcode & 0xF000 {
            //load index register with immediate value
            0x0000 => {
                // matches the 0x00EE and 0x000E opcodes
                match opcode & 0x000F {
                    0x0000 => {
                        //clear the screen
                        rip8.clear();
                        rip8.pc += 2;
                    }
                    0x000E => {
                        //return from subroutine
                        rip8.pc = rip8.stack[rip8.sp as usize] + 2;
                        rip8.sp -= 1;
                    }
                    _ => panic!("unknown opcode: {:#04x}", opcode),
                }
            }
            //doesn't increment pc
            0x1000 => {
                let address = opcode & 0x0FFF;
                rip8.pc = address;
            }
            //doesn't increment pc
            0x2000 => {
                let address: u16 = opcode & 0x0FFF;
                rip8.sp += 1;
                rip8.stack[rip8.sp as usize] = rip8.pc;
                rip8.pc = address;
            }
            0x3000 => {
                let value = opcode & 0x00FF;
                //skip next instruction if value is equal to value stored in register
                if reg_x_value == value as u8 {
                    rip8.pc += 2;
                }
                rip8.pc += 2;
            }
            0x4000 => {
                let value = opcode & 0x00FF;
                //skip next instruction if the value is not equal to the value stored in the
                //register
                if reg_x_value != value as u8 {
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
                rip8.registers[usize::from(reg_x_index)] = value as u8;
                rip8.pc += 2;
            }
            0x7000 => {
                let value = opcode & 0x00FF;
                rip8.registers[usize::from(reg_x_index)] = reg_x_value.wrapping_add(value as u8);
                rip8.pc += 2;
            }
            0x8000 => {
                // all increment PC by 2
                match opcode & 0x000F {
                    0x0000 => rip8.registers[usize::from(reg_x_index)] = reg_y_value,
                    0x0001 => rip8.registers[usize::from(reg_x_index)] = reg_x_value | reg_y_value,
                    0x0002 => rip8.registers[usize::from(reg_x_index)] = reg_x_value & reg_y_value,
                    0x0003 => rip8.registers[usize::from(reg_x_index)] = reg_x_value ^ reg_y_value,
                    0x0004 => {
                        //
                        //checks to see if the
                        let output: u16 = reg_x_value as u16 + reg_y_value as u16;
                        //some bit math to store the carry of an aaddition in Vf
                        rip8.registers[usize::from(reg_x_index)] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = ((output >> 8) & 0x01) as u8;
                    }
                    //maybe wrong implementation of negatives?
                    0x0005 => {
                        //skip next instruction if the values are the same in Vx and Vy
                        let (output, overflow) = reg_x_value.overflowing_sub(reg_y_value);
                        rip8.registers[usize::from(reg_x_index)] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = !overflow as u8;
                    }
                    0x0006 => {
                        // order of this matters ?? for the 4-test rom
                        rip8.registers[usize::from(reg_x_index)] = reg_x_value >> 1;
                        rip8.registers[0xF as usize] = reg_x_value & 0x1;
                    }
                    0x0007 => {
                        let (output, overflow) = reg_y_value.overflowing_sub(reg_x_value);
                        rip8.registers[usize::from(reg_x_index)] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = !overflow as u8;
                    }
                    0x000E => {
                        let (output, overflow) = reg_x_value.overflowing_mul(2);
                        rip8.registers[usize::from(reg_x_index)] = output;
                        rip8.registers[0xF as usize] = overflow as u8;
                    }
                    _ => panic!("unknown upcode"),
                }
                rip8.pc += 2;
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
                rip8.registers[usize::from(reg_x_index)] = rng & value;
                rip8.pc += 2;
            }
            //load register with immediate value
            //draw sprite to screen
            0xD000 => {
                rip8.registers[0xF as usize] = 0;

                let n = (opcode & 0x000F) as u8;
                let mut collision: bool = false;
                for mem_offset in 0..n {
                    //integer value for sprite byte stored in memory
                    let sprite = rip8.buffer[(rip8.i + mem_offset as u16) as usize];
                    for sprite_offset in 0..8 {
                        //if any pixel causes a collision set collision to true
                        if (sprite >> sprite_offset) & 1 == 1 {
                            let mut x_wrap: usize = (reg_x_value + 7 - sprite_offset) as usize;
                            let mut y_wrap: usize = (reg_y_value + mem_offset) as usize;
                            if x_wrap >= 64 {
                                x_wrap = (x_wrap % 64).try_into().unwrap();
                            }
                            if y_wrap >= 32 {
                                y_wrap = (y_wrap % 32).try_into().unwrap();
                            }
                            // swaps the bit at the correct coordinate
                            collision = rip8.display[y_wrap][x_wrap] | collision;
                            rip8.display[y_wrap][x_wrap] = !rip8.display[y_wrap][x_wrap];
                        }
                    }
                }
                rip8.registers[0xF as usize] = collision as u8;
                rip8.pc += 2;
            }
            //keyboard related opcodes
            0xE000 => {
                // all increment PC by 2
                //TODO
                match opcode & 0x00FF {
                    0x009E => {
                        if rip8.keydown[reg_x_value as usize] {
                            rip8.pc += 2;
                        }
                    }
                    0x00A1 => {
                        if !(rip8.keydown[reg_x_value as usize]) {
                            rip8.pc += 2;
                        }
                    }
                    _ => panic!("unknown opcode: {:#04x}", opcode),
                }
                rip8.pc += 2;
            }

            0xF000 => {
                // all increment PC by 2
                match opcode & 0x00FF {
                    0x0007 => {
                        //TODO
                        rip8.registers[reg_x_index as usize] = rip8.delay;
                    }
                    0x000A => {
                        //"halts" the program until a ky is pressed
                        //println!("current keypress: {}", rip8.keypress);
                        if !self.halted {
                            self.halted = true;
                            rip8.keypress = 0xFF;
                            for i in 0..rip8.keydown.len() {
                                rip8.keydown[i] = false
                            }
                        }
                        if rip8.keypress == 0xFF {
                            rip8.pc -= 2;
                        } else {
                            rip8.registers[reg_x_index as usize] = rip8.keypress as u8;
                            self.halted = false;
                            rip8.keypress = 0xFF;
                        }
                    }
                    0x0015 => rip8.delay = reg_x_value,
                    0x0018 => rip8.sound = reg_x_value,
                    0x001E => rip8.i += reg_x_value as u16,
                    0x0029 => rip8.i = (0x50 + (reg_x_value * 5)) as u16,
                    0x0033 => {
                        // really crude implementation without using the shift-add-three algo that
                        // i will implement in the future (section borrowed from
                        // https://github.com/aquova/chip8-book/blob/master/code/chip8_core/src/lib.rs)
                        // https://en.wikipedia.org/wiki/Double_dabble
                        let hundreds = (reg_x_value / 100) as u8;
                        let tens = ((reg_x_value / 10) % 10) as u8;
                        let ones = (reg_x_value % 10) as u8;

                        rip8.buffer[rip8.i as usize] = hundreds;
                        rip8.buffer[(rip8.i + 1) as usize] = tens;
                        rip8.buffer[(rip8.i + 2) as usize] = ones;
                    }
                    // reading and loading register values from memory
                    0x0055 => {
                        for x in 0..=reg_x_index {
                            rip8.buffer[rip8.i as usize] = rip8.registers[x as usize];
                            rip8.i += 1;
                        }
                    }
                    0x0065 => {
                        for x in 0..=reg_x_index {
                            rip8.registers[x as usize] = rip8.buffer[rip8.i as usize];
                            rip8.i += 1;
                        }
                    }
                    _ => panic!("unknown opcode: {:#04x}", opcode),
                }
                rip8.pc += 2;
            }
            _ => panic!("unknown opcode: {:#04x}", opcode),
        }

        if rip8.delay > 0 {
            if self.delay_state as u32 == self.timer_interval {
                rip8.delay -= 1;
                self.delay_state = 0;
            }
        }

        if rip8.sound > 0 {
            if self.sound_state as u32 == self.timer_interval {
                rip8.sound -= 1;
                self.sound_state = 0;
            }
        }
    }
}
