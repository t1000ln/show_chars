#![cfg_attr(
all(
target_os = "windows",
not(debug_assertions),
),
windows_subsystem = "windows"
)]

mod ui_loader;
// mod ascii;
mod fltk_color;
mod ansi_8_color;
// mod emoji;
mod html_color;
mod characters;
mod ascii;
mod emoji;

#[cfg(target_os = "linux")]
use std::path::PathBuf;

use fltk::app::App;
use fltk::enums::Font;
use fltk::image::{PngImage};
use fltk::prelude::WindowExt;
use crate::ansi_8_color::init_ansi_8_color;
use crate::ascii::init_ascii_tab;
use crate::characters::init_characters;
use crate::emoji::init_emoji;
use crate::fltk_color::init_fltk_color;
use crate::html_color::init_html_color;
use crate::ui_loader::UserInterface;

// pub const ICON_SVG: &str = include_str!("../resource/zhongwen.svg");
pub const ICON_PNG: &[u8] = include_bytes!("../resource/ma.png");


fn main() {
    let app_ins = App::default().load_system_fonts();

    #[cfg(target_os = "windows")]
    {
        Font::set_font(Font::Symbol, " Segoe UI Emoji");
        Font::set_font(Font::Screen, " Consolas");
        Font::set_font(Font::ScreenBold, "BConsolas");

    }

    #[cfg(target_os = "linux")]
    {
        let mut font_path = PathBuf::from("resource");
        font_path.push("NotoColorEmoji.ttf");
        if Font::load_font(font_path).is_ok() {
            Font::set_font(Font::Symbol, "Noto Color Emoji");
            // println!("{:#?}", app::get_font_names());
        }
    }

    let mut ui: UserInterface = UserInterface::make_window();

    if let Ok(icon) = PngImage::from_data(ICON_PNG) {
        ui.win_main.set_icon(Some(icon));
    }

    init_ascii_tab(&mut ui);
    init_emoji(&mut ui);
    init_fltk_color(&mut ui);
    init_ansi_8_color(&mut ui);
    init_html_color(&mut ui);
    init_characters(&mut ui);

    app_ins.run().unwrap();
}
