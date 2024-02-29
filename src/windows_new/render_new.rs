use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;



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
    pub total_time_millis: u32,
    pub num_frame: u32,
    pub frame_rate_sampled: f32,
    pub ipf_sampled: f32,
    pub ipf_count: u32,
    pub elapsed_time: u32,
}


pub fn render() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    


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



    // Set up an event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw the chessboard
        draw_emulator_screen(&mut canvas);

        // Present the canvas
        canvas.present();
    }
}

fn calc_pixel_size(window_width: u32, window_height: u32) -> u32 {

    std::cmp::min(
        window_width / EMULATOR_RES_W,
        window_height / EMULATOR_RES_H
    )
}

fn draw_emulator_screen(canvas: &mut Canvas<Window>) {

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
    let offset_height: u32 = match physical_window_height as i32 - (EMULATOR_RES_H * pixel_size) as i32 {
        x if x >= 0 => (x / 2) as u32,
        _ => 0
    };

    let offset_width: u32 = match physical_window_width as i32 - (EMULATOR_RES_W * pixel_size) as i32 {
        x if x >= 0 => (x / 2) as u32,
        _ => 0
    };


    for i in 0..EMULATOR_RES_W {
        for j in 0..EMULATOR_RES_H {
            let x = i * pixel_size + offset_width;
            let y = j * pixel_size + offset_height;

            // Alternate colors for the chessboard squares
            let color = if (i + j) % 2 == 0 {
                Color::RGB(200, 200, 200)
            } else {
                Color::RGB(100, 100, 100)
            };

            // Draw the square
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(x as i32, y as i32, pixel_size, pixel_size)).unwrap();
        }
    }
}

