//extern crate sdl2;
use std::env;
//use sdl2::event::Event;
use egui_backend::egui::FullOutput;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use sdl2::keyboard::Keycode;
use sdl2::video::SwapInterval;

use std::time::Instant;
use std::time::Duration;

use egui_sdl2_gl as egui_backend;

use crate::windows::gui::{draw_gui, set_gui_style};
use super::cpu::Cpu;
use super::keyboard::handle_key_event;
use super::rip8::Rip8;

const PIXEL_SIZE: u32 = 20;
const EMULATOR_WIDTH: u32 = 64;
const EMULATOR_HEIGHT: u32 = 32;
const TARGET_FPS: u32 = 60;
const CLOCK_SPEED: u32 = 700;

const DPI: u32 = 1;

pub struct DebugInfo {
    pub debug_active: bool,
    pub paused: bool,
    pub total_time_millis: u32,
    pub num_frame: u32,
    pub frame_rate_sampled: f32,
    pub ipf_sampled: f32,
    pub ipf_count: u32,
    pub elapsed_time: u32,
}


pub fn start_chip() {
    // loads the program
    let args: Vec<String> = env::args().collect();

    let mut rip8 = Rip8::new();
    let rom = args[1].to_string();

    rip8.load_program(rom.clone()).unwrap();

    // Calculate the window size based on the pixel size
    let window_size = (EMULATOR_WIDTH * PIXEL_SIZE, EMULATOR_HEIGHT * PIXEL_SIZE + 40);

    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);

    // setting up the window
    let window = video_subsystem
        .window("Chip-8 Emulator", window_size.0, window_size.1)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    //let canvas = window.into_canvas().build().unwrap();

    let _ctx = window.gl_create_context().unwrap();
    let shader_ver = ShaderVersion::Default;
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, shader_ver, DpiScaling::Custom(DPI as f32));
    let egui_ctx = egui::Context::default();

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::Immediate)
        .unwrap();

    //let mut canvas = window.into_canvas().build().unwrap();

    let mut cpu = Cpu {
        clock_speed: CLOCK_SPEED,
        timer_interval: CLOCK_SPEED / TARGET_FPS,
        delay_state: 0,
        sound_state: 0,
        halted: false,
    };

    // debug stuff
    let mut debug_info = DebugInfo {
        debug_active: false,
        paused: false,
        total_time_millis: 0,
        num_frame: 0,
        frame_rate_sampled: 0.0,
        ipf_sampled: 0.0,
        ipf_count: 0,
        elapsed_time: 0,
    };

    egui_ctx.set_style(set_gui_style());

    let start_time = Instant::now();

    'running: loop {
        let frame_time = Instant::now();

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        //draws the egui to the display
        draw_gui(
            &mut rip8,
            &egui_ctx,
            &mut debug_info,
            EMULATOR_WIDTH,
            EMULATOR_HEIGHT,
        );

        let FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        egui_state.process_output(&window, &platform_output);

        let paint_jobs = egui_ctx.tessellate(shapes);
        painter.paint_jobs(None, textures_delta, paint_jobs);

        window.gl_swap_window();

        // Handle events
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => debug_info.paused = !debug_info.paused,
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    if debug_info.paused {
                        cpu.emulate_cycle(&mut rip8);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Y),
                    ..
                } => {
                    rip8 = Rip8::new();
                    rip8.load_program(rom.clone()).unwrap();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    // Process input event
                    handle_key_event(&mut rip8, event.clone());
                    egui_state.process_input(&window, event.clone(), &mut painter);
                }
            }
        }

        //canvas.clear();

        //emulates x cycles per frame, decreases the timer each frame
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



         //debug related calculations
        if debug_info.num_frame == TARGET_FPS {

            debug_info.ipf_sampled =
             debug_info.ipf_count as f32 / (debug_info.total_time_millis as f32 * 0.001);
            
            debug_info.frame_rate_sampled = TARGET_FPS as f32 / (debug_info.total_time_millis as f32 / 1_000f32);
            debug_info.ipf_count = 0;
            debug_info.total_time_millis = 0;
            debug_info.num_frame = 0;
        }

        debug_info.elapsed_time = frame_time.elapsed().as_millis() as u32;

        if debug_info.elapsed_time < 17 {
            std::thread::sleep(Duration::new(0, (1_000_000_000u32 / 60) - (debug_info.elapsed_time * 1_000_000u32)));
        }

        debug_info.num_frame += 1;
        debug_info.elapsed_time = frame_time.elapsed().as_millis() as u32;
        debug_info.total_time_millis += debug_info.elapsed_time;

    }
}
