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
            0x0000 => {
                match opcode & 0x00FF {
                    0x00E0 => {
                        //clear the screen
                        rip8.clear_display();
                    }
                    0x00EE => {
                        //return from subroutine
                        rip8.sp -= 1;
                        rip8.pc = rip8.stack[rip8.sp as usize];
                    }
                    _ => panic!("unknown opcode: {:#04x}", opcode),
                }
                rip8.pc += 2;
            }
            0x1000 => rip8.pc = opcode & 0x0FFF,
            0x2000 => {
                rip8.stack[rip8.sp as usize] = rip8.pc;
                rip8.sp += 1;
                rip8.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                let value = opcode & 0x00FF;
                if reg_x_value == value as u8 {
                    rip8.pc += 2;
                }
                rip8.pc += 2;
            }
            0x4000 => {
                let value = opcode & 0x00FF;
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
                rip8.registers[reg_x_index as usize] = (opcode & 0x00FF) as u8;
                rip8.pc += 2;
            }
            0x7000 => {
                let value = opcode & 0x00FF;
                rip8.registers[reg_x_index as usize] = reg_x_value.wrapping_add(value as u8);
                rip8.pc += 2;
            }
            0x8000 => {
                // all increment PC by 2
                match opcode & 0x000F {
                    0x0000 => rip8.registers[reg_x_index as usize] = reg_y_value,
                    0x0001 => {
                        rip8.registers[reg_x_index as usize] = reg_x_value | reg_y_value;
                        rip8.registers[0xF] = 0;
                    }
                    0x0002 => {
                        rip8.registers[reg_x_index as usize] = reg_x_value & reg_y_value;
                        rip8.registers[0xF] = 0;
                    }
                    0x0003 => {
                        rip8.registers[reg_x_index as usize] = reg_x_value ^ reg_y_value;
                        rip8.registers[0xF] = 0;
                    }
                    0x0004 => {
                        let (output, overflow) = reg_x_value.overflowing_add(reg_y_value);
                        rip8.registers[reg_x_index as usize] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = overflow as u8;
                    }
                    0x0005 => {
                        //skip next instruction if the values are the same in Vx and Vy
                        let (output, overflow) = reg_x_value.overflowing_sub(reg_y_value);
                        rip8.registers[reg_x_index as usize] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = !overflow as u8;
                    }
                    0x0006 => {
                        // vy shl vx is a quirk
                        rip8.registers[reg_x_index as usize] = reg_y_value >> 1;
                        rip8.registers[0xF as usize] = reg_y_value & 0x1;
                    }
                    0x0007 => {
                        let (output, overflow) = reg_y_value.overflowing_sub(reg_x_value);
                        rip8.registers[reg_x_index as usize] = (output & 0xFF) as u8;
                        rip8.registers[0xF as usize] = !overflow as u8;
                    }
                    0x000E => {
                        // vy shl vx is a quirk
                        rip8.registers[reg_x_index as usize] = reg_y_value << 1;
                        rip8.registers[0xF as usize] = reg_y_value >> 7;
                    }
                    _ => panic!("unknown upcode"),
                }
                rip8.pc += 2;
            }
            0x9000 => {
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
            }
            0xC000 => {
                let value = (opcode & 0x00FF) as u8;
                let rng: u8 = rand::thread_rng().gen_range(0..=255);
                rip8.registers[reg_x_index as usize] = rng & value;
                rip8.pc += 2;
            }
            //draw sprite to screen
            0xD000 => {
                rip8.registers[0xF as usize] = 0;

                let nibble = (opcode & 0x000F) as u8;
                let mut collision: bool = false;

                let mut x_wrap: u8 = reg_x_value;
                let mut y_wrap: u8 = reg_y_value;

                // clipping quirk handled here
                if x_wrap > 63 {
                    x_wrap = (x_wrap % 64).try_into().unwrap();
                }
                if y_wrap > 31 {
                    y_wrap = (y_wrap % 32).try_into().unwrap();
                }

                for mem_offset in 0..nibble {
                    let sprite = rip8.buffer[(rip8.i + mem_offset as u16) as usize].reverse_bits();
                    let mut y = (y_wrap + mem_offset) as usize;
                    for sprite_offset in 0..8 {
                        if (sprite >> sprite_offset) & 0x1 == 1 {
                            let mut x = (x_wrap + sprite_offset) as usize;

                            // remove when not clipping
                            if x > 63 {
                                break;
                            }
                            if y > 31 {
                                break;
                            }

                            x = (x % 64).try_into().unwrap();
                            y = (y % 32).try_into().unwrap();

                            // xor's the bit
                            collision = rip8.display[y][x] | collision;
                            rip8.display[y][x] = true ^ rip8.display[y][x];
                        }
                    }
                }
                rip8.registers[0xF as usize] = collision as u8;
                rip8.pc += 2;
            }
            //keyboard related opcodes
            0xE000 => {
                // all increment PC by 2
                match opcode & 0x00FF {
                    0x009E => {
                        if rip8.keydown[(reg_x_value & 0xF) as usize] {
                            rip8.pc += 2;
                        }
                    }
                    0x00A1 => {
                        if !(rip8.keydown[(reg_x_value & 0xF) as usize]) {
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
                        rip8.registers[reg_x_index as usize] = rip8.delay;
                    }
                    0x000A => {
                        //"halts" the program until a ky is pressed
                        //println!("current keypress: {}", rip8.keypress);
                        if !self.halted {
                            self.halted = true;
                            rip8.keypress = 0xFF;
                        }
                        if rip8.keypress == 0xFF {
                            rip8.pc -= 2;
                        } else {
                            rip8.registers[reg_x_index as usize] = (rip8.keypress & 0xF) as u8;
                            self.halted = false;
                        }
                    }
                    0x0015 => rip8.delay = reg_x_value,
                    0x0018 => rip8.sound = reg_x_value,
                    0x001E => rip8.i += reg_x_value as u16,
                    0x0029 => rip8.i = (0x50 + ((reg_x_value & 0xF) * 5)) as u16,
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
    }
}
