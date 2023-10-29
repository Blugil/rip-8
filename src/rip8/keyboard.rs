use ::sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::rip8::Rip8;

pub fn handle_key_event(rip8: &mut Rip8, event: sdl2::event::Event) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::Num1),
            ..
        } => {
            rip8.keydown[0x1] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Num2),
            ..
        } => {
            rip8.keydown[0x2] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Num3),
            ..
        } => {
            rip8.keydown[0x3] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Num4),
            ..
        } => {
            rip8.keydown[0xC] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Q),
            ..
        } => {
            rip8.keydown[0x4] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => {
            rip8.keydown[0x5] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::E),
            ..
        } => {
            rip8.keydown[0x6] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
        } => {
            rip8.keydown[0xD] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            rip8.keydown[0x7] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            rip8.keydown[0x8] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            rip8.keydown[0x9] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::F),
            ..
        } => {
            rip8.keydown[0xE] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Z),
            ..
        } => {
            rip8.keydown[0xA] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::X),
            ..
        } => {
            rip8.keydown[0x0] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::C),
            ..
        } => {
            rip8.keydown[0xB] = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::V),
            ..
        } => {
            rip8.keydown[0xF] = true;
        }
        Event::KeyUp {
            keycode: Some(Keycode::Num1),
            ..
        } => {
            rip8.keydown[0x1] = false;
            rip8.keypress = 0x1;
        }
        Event::KeyUp {
            keycode: Some(Keycode::Num2),
            ..
        } => {
            rip8.keypress = 0x2;
            rip8.keydown[0x2] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::Num3),
            ..
        } => {
            rip8.keypress = 0x3;
            rip8.keydown[0x3] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::Num4),
            ..
        } => {
            rip8.keypress = 0xC;
            rip8.keydown[0xC] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::Q),
            ..
        } => {
            rip8.keypress = 0x4;
            rip8.keydown[0x4] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::W),
            ..
        } => {
            rip8.keypress = 0x5;
            rip8.keydown[0x5] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::E),
            ..
        } => {
            rip8.keypress = 0x6;
            rip8.keydown[0x6] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::R),
            ..
        } => {
            rip8.keypress = 0xD;
            rip8.keydown[0xD] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::A),
            ..
        } => {
            rip8.keypress = 0x7;
            rip8.keydown[0x7] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::S),
            ..
        } => {
            rip8.keypress = 0x8;
            rip8.keydown[0x8] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::D),
            ..
        } => {
            rip8.keypress = 0x9;
            rip8.keydown[0x9] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::F),
            ..
        } => {
            rip8.keypress = 0xE;
            rip8.keydown[0xE] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::Z),
            ..
        } => {
            rip8.keypress = 0xA;
            rip8.keydown[0xA] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::X),
            ..
        } => {
            rip8.keypress = 0x0;
            rip8.keydown[0x0] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::C),
            ..
        } => {
            rip8.keypress = 0xB;
            rip8.keydown[0xB] = false;
        }
        Event::KeyUp {
            keycode: Some(Keycode::V),
            ..
        } => {
            rip8.keypress = 0xF;
            rip8.keydown[0xF] = false;
        }
        //Event::KeyDown {
        //    keycode: Some(Keycode::P),
        //    ..
        //} => {
        //    rip8.pc = 0x200;
        //    rip8.i = 0x200;
        //    rip8.clear();
        //}
        _ => {}
    }
}
