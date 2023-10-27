//extern crate sdl2;

//use sdl2::event::Event;
use egui_backend::egui::FullOutput;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use sdl2::video::SwapInterval;
use std::time::Instant;

use sdl2::keyboard::Keycode;

use std::time::Duration;

use egui_sdl2_gl as egui_backend;

use super::cpu::Cpu;
use super::gui::{draw_bottom_panel, draw_game_window, draw_side_panel};
use super::keyboard::handle_key_event;
use super::rip8::Rip8;

//const PIXEL_SIZE: u32 = 32; // Size of each pixel in pixels
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

pub fn create_window(rip8: &mut Rip8) {
    // Calculate the window size based on the pixel size
    let window_size = (1800, 1200);

    let clock_speed = 1000;
    let fps = 60;
    let timer_interval = clock_speed / fps;

    // Initialize SDL

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);

    let window = video_subsystem
        .window(
            "Demo: Egui backend for SDL2 + GL",
            window_size.0,
            window_size.1,
        )
        .opengl()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let _ctx = canvas.window().gl_create_context().unwrap();
    let shader_ver = ShaderVersion::Adaptive;
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&canvas.window(), shader_ver, DpiScaling::Custom(4.0));
    let egui_ctx = egui::Context::default();

    let start_time = Instant::now();

    canvas
        .window()
        .subsystem()
        .gl_set_swap_interval(SwapInterval::Immediate)
        .unwrap();

    let mut cpu = Cpu {
        clock_speed,
        timer_interval,
        delay_state: 0,
        sound_state: 0,
        halted: false,
    };

    // main loop
    'running: loop {
        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        draw_side_panel(rip8, &egui_ctx);
        draw_bottom_panel(&egui_ctx);
        draw_game_window(rip8, &egui_ctx, SCREEN_HEIGHT, SCREEN_WIDTH);

        let FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        egui_state.process_output(&canvas.window(), &platform_output);

        let paint_jobs = egui_ctx.tessellate(shapes);

        // Handle events
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {
                    // Process input event
                    handle_key_event(rip8, event.clone());
                    egui_state.process_input(&canvas.window(), event.clone(), &mut painter);
                }
            }
        }

        canvas.clear();

        painter.paint_jobs(None, textures_delta, paint_jobs);
        canvas.window().gl_swap_window();

        for _ in 0..timer_interval {
            cpu.emulate_cycle(rip8);
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }
}
