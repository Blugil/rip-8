use std::env;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{SwapInterval, Window};

use crate::rip8::cpu::Cpu;
use crate::rip8::keyboard::handle_key_event;
use crate::rip8::rip8::Rip8;
use crate::windows::debug;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const EMULATOR_RES_W: u32 = 64;
const EMULATOR_RES_H: u32 = 32;

const PIXEL_SIZE: u32 = 5;

const TARGET_FRAMERATE: u32 = 60;

const TARGET_FPS: u32 = 60;
const CLOCK_SPEED: u32 = 700;

pub struct DebugInfo {
    pub debug_active: bool,
    pub paused: bool,
    pub total_time_nanos: u32,
    pub num_frame: u32,
    pub frame_rate_sampled: f32,
    pub ipf_sampled: f32,
    pub ipf_count: u32,
    pub elapsed_time: u32,
}

pub fn render() {
    let args: Vec<String> = env::args().collect();

    let mut rip8 = Rip8::new();
    let rom = args[1].to_string();

    rip8.load_program(rom.clone()).unwrap();

    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut cpu = Cpu {
        clock_speed: CLOCK_SPEED,
        timer_interval: CLOCK_SPEED / TARGET_FPS,
        delay_state: 0,
        sound_state: 0,
        halted: false,
    };

    let mut debug_info = DebugInfo {
        debug_active: false,
        paused: false,
        total_time_nanos: 0,
        num_frame: 0,
        frame_rate_sampled: 0.0,
        ipf_sampled: 0.0,
        ipf_count: 0,
        elapsed_time: 0,
    };

    let start_time = Instant::now();

    // Set up an event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        let frame_time = Instant::now();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        draw_emulator_screen(&mut canvas, &rip8);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {
                    // Process input event
                    handle_key_event(&mut rip8, event.clone());
                }
            }
        }

        if !debug_info.paused {
            if rip8.delay > 0 {
                rip8.delay -= 1;
            }
            if rip8.sound > 0 {
                rip8.sound -= 1;
            }

            for _ in 0..cpu.timer_interval + 1 {
                cpu.emulate_cycle(&mut rip8);
                debug_info.ipf_count += 1;
            }
        }

        // Present the canvas
        canvas.present();

        debug_info.num_frame += 1;
        if debug_info.num_frame == TARGET_FPS {
            debug_info.ipf_sampled =
                debug_info.ipf_count as f32 / (debug_info.total_time_nanos as f32 * 0.000000001);

            debug_info.frame_rate_sampled =
                TARGET_FPS as f32 / (debug_info.total_time_nanos as f32 / 1_000_000_000f32);
            debug_info.ipf_count = 0;
            debug_info.total_time_nanos = 0;
            debug_info.num_frame = 0;
        }

        debug_info.elapsed_time = frame_time.elapsed().as_nanos() as u32;
        if debug_info.elapsed_time < 1_000_000_000u32 / 60 {
            std::thread::sleep(Duration::new(
                0,
                (1_000_000_000u32 / 64) - (debug_info.elapsed_time),
            ));
        }

        debug_info.elapsed_time = frame_time.elapsed().as_nanos() as u32;
        debug_info.total_time_nanos += debug_info.elapsed_time;
    }
}

fn calc_pixel_size(window_width: u32, window_height: u32) -> u32 {
    std::cmp::min(
        window_width / EMULATOR_RES_W,
        window_height / EMULATOR_RES_H,
    )
}

fn draw_emulator_screen(canvas: &mut Canvas<Window>, rip8: &Rip8) {
    let physical_window_height: u32 = canvas.window().drawable_size().1;
    let physical_window_width: u32 = canvas.window().drawable_size().0;

    // TODO:
    // offset calculation and position determination, so far it's just calculating for a centered
    // position but i might want it to have multiple positibilities(?)
    //
    // also need to determine a "fullscreen" mode which is coupled with centered to create a
    // scaling virtual window that scales with physical window size screen size
    //

    // fullscreen mode: scales the size to match the maximum amount
    let pixel_size = calc_pixel_size(physical_window_width, physical_window_height);

    // offset for centering the virtual window
    let offset_height: u32 =
        match physical_window_height as i32 - (EMULATOR_RES_H * pixel_size) as i32 {
            x if x >= 0 => (x / 2) as u32,
            _ => 0,
        };

    let offset_width: u32 =
        match physical_window_width as i32 - (EMULATOR_RES_W * pixel_size) as i32 {
            x if x >= 0 => (x / 2) as u32,
            _ => 0,
        };

    for i in 0..EMULATOR_RES_W {
        for j in 0..EMULATOR_RES_H {
            let x = i * pixel_size + offset_width;
            let y = j * pixel_size + offset_height;

            // Alternate colors for the chessboard squares
            let color = if rip8.display[j as usize][i as usize] {
                Color::RGB(140, 89, 77)
            } else {
                Color::RGB(14, 14, 14)
            };

            // Draw the square
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(x as i32, y as i32, pixel_size, pixel_size))
                .unwrap();
        }
    }
}
