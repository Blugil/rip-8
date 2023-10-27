use egui_backend::egui;
use egui_backend::egui::Style;

use egui_sdl2_gl as egui_backend;

use super::rip8::Rip8;

pub fn draw_side_panel(rip8: &Rip8, egui_ctx: &egui::Context) {
    egui::SidePanel::right("right panel")
        .resizable(false)
        .show(&egui_ctx, |ui| {
            ui.set_width(400.0);
            ui.label(" ");
            ui.horizontal_centered(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.set_width(ui.available_size()[0] / 2.0);
                    ui.label("V[x]");
                    ui.separator();
                    ui.label(format!(
                        "{}",
                        rip8.registers
                            .iter()
                            .enumerate()
                            .map(|(index, &value)| format!("V{:X}: {:#04x}", index, value))
                            .collect::<Vec<String>>()
                            .join("\n")
                    ));
                    ui.label(format!("PC: {:#04x}", rip8.pc));
                    ui.label(format!("I: {:#04x}", rip8.i));
                    ui.label(format!("DT: {:#04}", rip8.delay));
                    ui.label(format!("ST: {:#04}", rip8.sound));
                    ui.label(format!("SP: {:#04x}", rip8.sp));
                });
                ui.separator();
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.label("stack");
                    ui.separator();
                    ui.label(format!(
                        "{}",
                        rip8.stack
                            .iter()
                            .map(|&value| format!("{:#04x}", value))
                            .collect::<Vec<String>>()
                            .join("\n")
                    ));
                });
            });
        });
}

pub fn draw_bottom_panel(egui_ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom panel").show(&egui_ctx, |ui| {
        ui.set_height(350.0);
        ui.label("Bottom panel");
        ui.available_size();
    });
}

pub fn draw_game_window(
    rip8: &Rip8,
    egui_ctx: &egui::Context,
    screen_height: u32,
    screen_width: u32,
) {
    egui::CentralPanel::default().show(&egui_ctx, |ui| {
        egui::Frame::dark_canvas(&Style::default()).show(ui, |ui| {
            let width = ui.available_size()[0];
            let height = ui.available_size()[1];
            let pixel_size = if (width / 64.0) >= (height / 32.0) {
                height / screen_height as f32
            } else {
                width / screen_width as f32
            };
            for y in 0..screen_height {
                for x in 0..screen_width {
                    let color = if rip8.display[y as usize][x as usize] {
                        egui::Color32::from_rgb(140, 89, 77)
                    } else {
                        egui::Color32::from_rgb(14, 14, 14)
                    };
                    let rect = egui::Rect::from_min_size(
                        ui.painter().round_pos_to_pixels(egui::pos2(x as f32 * pixel_size, y as f32 * pixel_size)),
                        ui.painter().round_vec_to_pixels(egui::vec2(pixel_size, pixel_size)),
                    );
                    ui.painter().rect_filled(rect, 0.0, color);
                }
            }
        });
    });
}
