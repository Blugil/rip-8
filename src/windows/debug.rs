use egui_backend::egui::{self, FontFamily, FontId, TextStyle};
use egui_sdl2_gl as egui_backend;

use crate::rip8::render::DebugInfo;
use crate::Rip8;


pub fn draw_debug_window(rip8: &Rip8, egui_ctx: &egui::Context, debug_info: &mut DebugInfo) {
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
