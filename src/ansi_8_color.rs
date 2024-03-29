use fltk::enums::{Color, FrameType};
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::prelude::{GroupExt, WidgetExt};
use crate::ui_loader::UserInterface;

pub fn init_ansi_8_color(ui: &mut UserInterface) {
    ui.flex_ansi_8_color.begin();

    init_216_color(ui);
    init_gray_color(ui);


    ui.flex_ansi_8_color.end();
}

fn init_216_color(ui: &mut UserInterface) {
    ui.flex_ansi_216.begin();
    let mut seq = 16;
    for r in 0..6 {
        let red_color = ((255f32 / 5f32) * r as f32).round() as u8;
        let mut row_flex = Flex::default().row();
        row_flex.set_frame(FrameType::FlatBox);
        // row_flex.set_margin(1);
        row_flex.set_pad(1);
        for g in 0..6 {
            let green_color = ((255f32 / 5f32) * g as f32).round() as u8;
            for b in 0..6 {
                let blue_color = ((255f32 / 5f32) * b as f32).round() as u8;
                let mut rect = Frame::default().with_label(seq.to_string().as_str());
                rect.set_frame(FrameType::FlatBox);
                rect.set_color(Color::from_rgb(red_color, green_color, blue_color));

                if r < 4 && g < 4 {
                    rect.set_label_color(Color::White);
                }
                seq += 1;
            }
        }

        row_flex.end();
        row_flex.recalc();
    }
    ui.flex_ansi_216.end();
    ui.flex_ansi_216.recalc();
}

fn init_gray_color(ui: &mut UserInterface) {
    for i in 0..ui.flex_ansi_gray.children() {
        if let Some(mut gray_box) = ui.flex_ansi_gray.child(i) {
            if i < 12 {
                gray_box.set_label_color(Color::White.darker());
            }
            let v = ((255f64 / 24f64) * i as f64).round() as u8;
            let color = Color::from_rgb(v, v, v);
            gray_box.set_color(color);
        }
    }
}