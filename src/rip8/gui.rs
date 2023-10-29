use egui_backend::egui;
use egui_backend::egui::Style;

use egui_sdl2_gl as egui_backend;

use super::rip8::Rip8;

pub fn draw_side_panel(rip8: &Rip8, egui_ctx: &egui::Context) {
    let opcode = u16::from(rip8.buffer.get(usize::from(rip8.pc)).unwrap().to_owned()) << 8
        | u16::from(
            rip8.buffer
                .get(usize::from(rip8.pc + 1))
                .unwrap()
                .to_owned(),
        );
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
                    ui.label(format!(" I: {:#04x}", rip8.i));
                    ui.label(format!("DT: {:#04}", rip8.delay));
                    ui.label(format!("ST: {:#04}", rip8.sound));
                    ui.label(format!("SP: {:#04x}", rip8.sp));
                    ui.label(" ");
                    ui.label(format!("OP: {:#04x}", opcode));
                });
                ui.separator();
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.label("Stack");
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

pub fn draw_bottom_panel(egui_ctx: &egui::Context, frame_rate: f32) {
    egui::TopBottomPanel::bottom("bottom panel").show(&egui_ctx, |ui| {
        ui.set_height(500.0);
        ui.label(" ");
        ui.label("Y: Reset. P: pause/resume.  O: Step Into.");
        ui.label(" ");
        ui.label(format!("Current framerate: {}", frame_rate));
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
                    let rect = egui::Rect::from_min_max(
                        ui.painter().round_pos_to_pixels(egui::pos2(
                            x as f32 * pixel_size,
                            y as f32 * pixel_size,
                        )),
                        ui.painter().round_pos_to_pixels(egui::pos2(
                            (x + 1) as f32 * pixel_size,
                            (y + 1) as f32 * pixel_size,
                        )),
                    );
                    ui.painter().rect_filled(rect, 0.0, color);
                }
            }
        });
    });
}
