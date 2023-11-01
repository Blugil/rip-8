use egui_backend::egui::Style;
use egui_backend::egui::{self, FontFamily, FontId, TextStyle, Ui};

use egui_sdl2_gl as egui_backend;

use super::render::DebugInfo;
use super::rip8::Rip8;

pub fn set_gui_style() -> Style {
    let mut style = Style::default();
    style.text_styles = [
        (
            TextStyle::Body,
            FontId::new(36.0 as f32, FontFamily::Monospace),
        ),
        (
            TextStyle::Heading,
            FontId::new(48.0 as f32, FontFamily::Monospace),
        ),
        (
            TextStyle::Button,
            FontId::new(36.0 as f32, FontFamily::Monospace),
        ),
    ]
    .into();

    style.spacing.icon_width = 36.0;

    style
}

pub fn draw_gui(
    rip8: &Rip8,
    egui_ctx: &egui::Context,
    debug_info: &mut DebugInfo,
    emulator_height: u32,
    emulator_width: u32,
) {
    draw_menu_bar(egui_ctx, &mut debug_info.debug_active);
    if debug_info.debug_active {
        draw_debug_window(rip8, egui_ctx, debug_info);
    }
    draw_game_window(rip8, egui_ctx, emulator_width, emulator_height);
}

fn draw_menu_bar(egui_ctx: &egui::Context, debug: &mut bool) {
    egui::TopBottomPanel::top("Menu bar").show(egui_ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.spacing_mut().button_padding = [48.0, 24.0].into();
            ui.spacing_mut().interact_size = [24.0, 24.0].into();
            ui.set_height(80.0);
            ui.menu_button("File", |ui| {
                ui.set_width(300.0);
                ui.label("Test");
                ui.separator();
                ui.button("Quit")
                    .on_hover_text("This is where we'd quit the program....IF I COULD");
            });
            ui.menu_button("View", |ui| {
                ui.set_width(300.0);
                ui.label("Debug info");
                ui.separator();
                ui.checkbox(debug, " Debug")
                    .on_hover_text("Set's the debug window to on or off.");
            });
        });
    });
}

fn draw_debug_window(rip8: &Rip8, egui_ctx: &egui::Context, debug_info: &mut DebugInfo) {
    let opcode = u16::from(rip8.buffer.get(usize::from(rip8.pc)).unwrap().to_owned()) << 8
        | u16::from(
            rip8.buffer
                .get(usize::from(rip8.pc + 1))
                .unwrap()
                .to_owned(),
        );
    egui::Window::new("Debug Window")
        .default_open(true)
        .collapsible(true)
        .resizable(false)
        .open(&mut debug_info.debug_active)
        .show(&egui_ctx, |ui| {
            ui.set_width(600.0);
            let column_width = ui.available_size()[0] / 3.0;
            ui.label(" ");
            ui.horizontal_centered(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.set_width(column_width);
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
                    ui.label(" ");
                });
                ui.separator();
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.set_width(column_width);
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
                ui.separator();
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.set_width(column_width * 2.0);
                    ui.label("Debug keys");
                    ui.separator();
                    ui.label("Y: Reset");
                    ui.label("P: pause/resume.");
                    ui.label("O: Step Into.");
                    ui.label(" ");
                    ui.label(format!("FPS: {}", debug_info.frame_rate_sampled));
                    ui.label(format!("Clock: {}Hz", debug_info.ipf_sampled));
                });
            });
        });
}

fn draw_game_window(rip8: &Rip8, egui_ctx: &egui::Context, screen_height: u32, screen_width: u32) {
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
