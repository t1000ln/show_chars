use fltk::prelude::{InputExt, WidgetExt};
use bitflags::parser::ParseHex;
use crate::ui_loader::UserInterface;

pub fn init_characters(ui: &mut UserInterface) {
    ui.in_char_str.set_callback({
        let mut out_str_plain = ui.out_char_codes.clone();
        let mut out_str_rust = ui.out_char_codes_rust.clone();
        let mut out_str_java = ui.out_char_codes_java.clone();
        move |input| {
            let v = input.value();
            if !v.is_empty() {
                let decoded = v.chars().map(|c| format!("{:X?}", c as u32)).collect::<Vec<_>>().join("  ");
                out_str_plain.set_value(decoded.as_str());

                let decoded_rust = v.chars().map(|c| c.escape_unicode().to_string()).collect::<Vec<_>>().join(" ");
                out_str_rust.set_value(decoded_rust.as_str());

                let decoded_java = v.chars().map(|c| format!("\\u{:X?}", c as u32)).collect::<Vec<_>>().join(" ");
                out_str_java.set_value(decoded_java.as_str());
            } else {
                out_str_rust.set_value(v.as_str());
            }
        }
    });

    ui.in_char_code.set_callback({
        let mut out_char = ui.out_char.clone();
        move |input| {
            let v = input.value();
            if !v.is_empty() {
                if let Ok(hex) = u32::parse_hex(v.as_str()) {
                    if let Some(c) = char::from_u32(hex) {
                        out_char.set_value(c.to_string().as_str());
                    }
                }
            } else {
                out_char.set_value(v.as_str());
            }
        }
    });
}