use egui_backend::egui::{self};
use egui_sdl2_gl as egui_backend;

pub fn draw_menu_bar(egui_ctx: &egui::Context, debug: &mut bool) {
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
