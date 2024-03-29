mod ui_loader;
mod ascii;
mod fltk_color;
mod ansi_8_color;
mod emoji;

#[cfg(target_os = "linux")]
use std::path::PathBuf;

use fltk::app::App;
use fltk::enums::Font;
use crate::ansi_8_color::init_ansi_8_color;
use crate::ascii::init_ascii_tab;
use crate::emoji::init_emoji;
use crate::fltk_color::init_fltk_color;
use crate::ui_loader::UserInterface;


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
            println!("{:#?}", app::get_font_names());
        }
    }


    let mut ui: UserInterface = UserInterface::make_window();

    init_ascii_tab(&mut ui);
    init_emoji(&mut ui);
    init_fltk_color(&mut ui);
    init_ansi_8_color(&mut ui);

    app_ins.run().unwrap();
}
