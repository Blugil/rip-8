use egui_backend::egui::{self, FontFamily, FontId, TextStyle};

use egui_sdl2_gl as egui_backend;

use crate::Rip8;


pub fn draw_game_window(rip8: &Rip8, egui_ctx: &egui::Context, screen_height: u32, screen_width: u32) {
    egui::CentralPanel::default().show(&egui_ctx, |ui| {
        //  calculates the x,y position of the top left corner of the current widget
        let x_start = ui.min_rect().left_top().x;
        let y_start = ui.min_rect().left_top().y;

        egui::Frame::dark_canvas(&egui_ctx.style()).show(ui, |ui| {
            let available_width = ui.available_size()[0];
            let available_height = ui.available_size()[1];

            let pixel_size = if (available_width / screen_width as f32)
                >= (available_height / screen_height as f32)
            {
                available_height / screen_height as f32
            } else {
                available_width / screen_width as f32
            };

            //draws the game display to the canvas
            for y in 0..screen_height {
                for x in 0..screen_width {
                    let color = if rip8.display[y as usize][x as usize] {
                        egui::Color32::from_rgb(140, 89, 77)
                    } else {
                        egui::Color32::from_rgb(14, 14, 14)
                    };
                    // the round functions fix those pesky gaps between pixels
                    let rect = egui::Rect::from_min_max(
                        ui.painter().round_pos_to_pixels(egui::pos2(
                            (x as f32) * pixel_size + x_start,
                            (y as f32) * pixel_size + y_start,
                        )),
                        ui.painter().round_pos_to_pixels(egui::pos2(
                            (x as f32 + 1.0) * pixel_size + x_start,
                            (y as f32 + 1.0) * pixel_size + y_start,
                        )),
                    );
                    ui.painter().rect_filled(rect, 0.0, color);
                }
            }
        });
    });
}
