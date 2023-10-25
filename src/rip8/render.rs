extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf;
use std::path::Path;
use std::time::Duration;

use super::cpu::Cpu;
use super::rip8::Rip8;

const PIXEL_SIZE: u32 = 30; // Size of each pixel in pixels
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;


pub fn create_window(rip8: &mut Rip8) {
    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Calculate the window size based on the pixel size
    let window_size = (SCREEN_WIDTH * PIXEL_SIZE, SCREEN_HEIGHT * PIXEL_SIZE + 200);

    // Create the window and canvas
    let window = video_subsystem
        .window("rip8", window_size.0, window_size.1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let clock_speed = 700;
    let fps = 60;
    let timer_interval = clock_speed / 60;

    let mut cpu = Cpu {
        clock_speed,
        timer_interval,
        delay_state: 0,
        sound_state: 0,
        halted: false,
    };


    //let texture = canvas.texture_creator();
    //let ttf_context = ttf::init().map_err(|e| e.to_string()).expect("unable to initialize font");

    //let font_path: &Path = Path::new(&"../../resources/fonts/standard-book-webfont.ttf");
    //let mut font = ttf_context.load_font(font_path, 128);
    

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    // main loop
    'running: loop {
        canvas.clear();
        // Handle events
        for event in sdl_context.event_pump().unwrap().poll_iter() {
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
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    rip8.pc = 0x200;
                    rip8.i = 0x200;
                    rip8.clear();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                // handles every other arm of the branch
                _ => {}
            }
        }

        // redraw the screen
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let pixel_rect = Rect::new(
                    x as i32 * PIXEL_SIZE as i32,
                    y as i32 * PIXEL_SIZE as i32,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                );
                if rip8.display[y as usize][x as usize] {
                    // draws a white "pixel" to the screen as a representation of the byte value in
                    // the dsplay array
                    canvas.set_draw_color(Color::RGB(140, 89, 77));
                    canvas.fill_rect(pixel_rect).unwrap();
                } else {
                    canvas.set_draw_color(Color::RGB(14, 14, 14));
                    canvas.fill_rect(pixel_rect).unwrap();
                }
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
        }

        // Present the canvas to the window
        // render the window at 60fps but keep the cpu at a normal clock
        canvas.present();
        for _ in 0..timer_interval {
            cpu.emulate_cycle(rip8);
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }
}
