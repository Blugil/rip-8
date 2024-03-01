use egui_backend::egui::Style;
use egui_backend::egui::{self, FontFamily, FontId, TextStyle};

use egui_sdl2_gl as egui_backend;


use crate::render::DebugInfo;
use crate::rip8::rip8::Rip8;

use crate::windows::game::draw_game_window;
use crate::windows::debug::draw_debug_window;
use crate::windows::menu::draw_menu_bar;

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

