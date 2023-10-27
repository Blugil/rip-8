//extern crate sdl2;

//use sdl2::event::Event;
use egui_backend::egui::{FullOutput, Style};
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use std::time::Instant;
use sdl2::video::SwapInterval;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;



use egui_sdl2_gl as egui_backend;


use super::cpu::Cpu;
use super::rip8::Rip8;

const PIXEL_SIZE: u32 = 32; // Size of each pixel in pixels
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;


pub fn create_window(rip8: &mut Rip8) {

    // Calculate the window size based on the pixel size
    let window_size = (SCREEN_WIDTH * PIXEL_SIZE + 100, SCREEN_HEIGHT * PIXEL_SIZE + 100);

    let clock_speed = 700;
    let fps = 60;
    let timer_interval = clock_speed / fps;

    let mut quit = false;
    
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

    let (mut painter, mut egui_state) = egui_backend::with_sdl2(&canvas.window(), shader_ver, DpiScaling::Custom(2.5));
    let egui_ctx = egui::Context::default();


    let start_time = Instant::now();


    canvas.window().subsystem().gl_set_swap_interval(SwapInterval::Immediate).unwrap();

    let mut cpu = Cpu {
        clock_speed,
        timer_interval,
        delay_state: 0,
        sound_state: 0,
        halted: false,
    };



    canvas.set_draw_color(Color::RGB(0, 0, 0));

    // main loop
    'running: loop {


        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        egui::CentralPanel::default()
            .show(&egui_ctx, |ui| {
            // Your Egui widgets here
            egui::Frame::dark_canvas(&Style::default())
                .fill(egui::Color32::BLACK)
                .show(ui, |ui| {
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        let color = if rip8.display[y as usize][x as usize] {
                            egui::Color32::from_rgb(140,89,77)
                        } else {
                            egui::Color32::from_rgb(14,14,14)
                        };

                        let rect = egui::Rect::from_min_max(
                            egui::pos2((x * PIXEL_SIZE) as f32, (y * PIXEL_SIZE) as f32),
                            egui::pos2(((x + 1) * PIXEL_SIZE) as f32, ((y + 1) * PIXEL_SIZE) as f32),
                        );

                        ui.painter().rect_filled(rect, 0.0, color);
                    }
                }
            });
        });


        egui::SidePanel::right("right panel")
            .resizable(false)
            .max_width(200.0)
            .show(&egui_ctx, |ui| {
                ui.label(" ");
                ui.label(format!("v0: {} \nv1: {}", rip8.registers[0x0], rip8.registers[0x1]));
                ui.separator();
                if ui.button("Quit?").clicked() {
                    quit = true;
                }
            }
        );

        egui::TopBottomPanel::bottom("bottom panel")
            .exact_height(200.0)
            .show(&egui_ctx, |ui| {
                ui.label("Bottom panel");
            }
        );




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
                Event::Quit { .. } => break 'running,
                _ => {
                    // Process input event
                    egui_state.process_input(&canvas.window(), event, &mut painter);
                }
            }
        }
        canvas.clear();

        // redraw the screen
        //for x in 0..SCREEN_WIDTH {
        //    for y in 0..SCREEN_HEIGHT {
        //        let pixel_rect = Rect::new(
        //            x as i32 * PIXEL_SIZE as i32,
        //            y as i32 * PIXEL_SIZE as i32,
        //            PIXEL_SIZE,
        //            PIXEL_SIZE,
        //        );
        //        if rip8.display[y as usize][x as usize] {
        //            // draws a white "pixel" to the screen as a representation of the byte value in
        //            // the dsplay array
        //            canvas.set_draw_color(Color::RGB(140, 89, 77));
        //            canvas.fill_rect(pixel_rect).unwrap();
        //        } else {
        //            canvas.set_draw_color(Color::RGB(14, 14, 14));
        //            canvas.fill_rect(pixel_rect).unwrap();
        //        }
        //        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //    }
        //}


        // render the window at 60fps but keep the cpu at a normal clock
        // Present the canvas to the window
        
        painter.paint_jobs(None, textures_delta, paint_jobs);

        canvas.window().gl_swap_window();

        //canvas.present();


        for _ in 0..timer_interval {
            cpu.emulate_cycle(rip8);
        }

        // quites on the button press
        if quit {
            break;
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }
}
