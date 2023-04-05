extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::rip8::Rip8;

const PIXEL_SIZE: u32 = 30; // Size of each pixel in pixels
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

pub fn create_window(rip8: &mut Rip8) {
    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Calculate the window size based on the pixel size
    let window_size = (SCREEN_WIDTH * PIXEL_SIZE, SCREEN_HEIGHT * PIXEL_SIZE);

    // Create the window and canvas
    let window = video_subsystem
        .window("Pixel Screen", window_size.0, window_size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Set the top-left pixel to on for testing
    rip8.invert_pixel(0, 0);

    // Main loop
    'running: loop {
        // Handle events
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::H),
                    ..
                } => {
                    rip8.invert_pixel(0, 0);
                }
                // handles every other arm of the branch
                _ => {}
            }
        }

        // Clear the canvas to black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                if rip8.display[y as usize][x as usize] {
                    // draws a white "pixel" to the screen as a representation of the byte value in
                    // the dsplay array
                    let pixel_rect = Rect::new(
                        x as i32 * PIXEL_SIZE as i32,
                        y as i32 * PIXEL_SIZE as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    );
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(pixel_rect).unwrap();
                }
            }
        }

        // Present the canvas to the window
        canvas.present();
    }
}
