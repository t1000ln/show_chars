use std::cmp::max;
use fltk::app;
use fltk::enums::{Color, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::output::Output;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;
use serde::{Deserialize, Serialize};
use crate::ui_loader::UserInterface;

pub const HTML_COLOR_STR: &str = include_str!("../resource/html_colors.json");

const HTML_COLOR_SPLIT_COLS: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorInfo {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
}

fn convert_to_2d(html_colors: Vec<ColorInfo>) -> Vec<Vec<ColorInfo>> {
    let height = max(html_colors.len() / HTML_COLOR_SPLIT_COLS, 1);
    let width = html_colors.len() / height;

    let mut result = Vec::with_capacity(width);

    for chunks in html_colors.chunks(width) {
        result.push(chunks.to_vec());
    }

    result
}

pub fn init_html_color(ui: &mut UserInterface) {
    add_header(ui);

    let html_colors = serde_json::from_str::<Vec<ColorInfo>>(HTML_COLOR_STR).unwrap();
    let colors_2d = convert_to_2d(html_colors);

    ui.pack_html_color.begin();
    ui.pack_html_color.set_spacing(2);

    for row in colors_2d.iter() {
        let row_flex = Flex::default().row().with_size(1, 32);
        for i in 0..HTML_COLOR_SPLIT_COLS {
            if let Some(ci) = row.get(i) {
                let mut block_flex = Flex::default().row();
                block_flex.set_frame(FrameType::ThinUpFrame);
                block_flex.set_color(ui.pack_html_color.color());
                block_flex.set_spacing(1);

                let mut rect0 = Frame::default();
                rect0.set_frame(FrameType::FlatBox);
                rect0.set_color(Color::from_rgb(ci.r, ci.g, ci.b));

                let mut o1 = Output::default();
                o1.set_frame(FrameType::FlatBox);
                o1.set_value(ci.name.as_str());
                o1.set_text_size(11);
                o1.set_cursor_color(o1.color());
                setup_output_handler(&mut o1, &mut ui.flex_echo_html_color);

                let mut o2 = Output::default();
                o2.set_frame(FrameType::FlatBox);
                o2.set_value(format!("{},{},{}", ci.r, ci.g, ci.b).as_str());
                o2.set_cursor_color(o2.color());
                o2.set_text_font(Font::Screen);
                setup_output_handler(&mut o2, &mut ui.flex_echo_html_color);

                let mut o3 = Output::default();
                o3.set_frame(FrameType::FlatBox);
                o3.set_value(ci.hex.as_str());
                o3.set_cursor_color(o3.color());
                o3.set_text_font(Font::Screen);
                setup_output_handler(&mut o3, &mut ui.flex_echo_html_color);

                block_flex.end();
                block_flex.recalc();
            } else {
                let mut empty_flex = Flex::default().row();
                empty_flex.set_frame(FrameType::NoBox);
                Frame::default();
                empty_flex.end();
                empty_flex.recalc();
            }
        }

        row_flex.end();
        row_flex.recalc();
    }

    ui.pack_html_color.end();


}

fn add_header(ui: &mut UserInterface) {
    ui.flex_echo_html_color.begin();

    let header_color = Color::by_index(52);
    for _ in 0..HTML_COLOR_SPLIT_COLS {
        let mut header_flex = Flex::default().row();
        header_flex.set_spacing(1);
        header_flex.set_frame(FrameType::FlatBox);
        header_flex.set_color(header_color);

        let mut header0 = Frame::default().with_label("颜色");
        header0.set_frame(FrameType::ThinUpBox);
        header0.set_color(header_color);

        let mut header1 = Frame::default().with_label("颜色名");
        header1.set_frame(FrameType::ThinUpBox);
        header1.set_color(header_color);

        let mut header2 = Frame::default().with_label("RGB");
        header2.set_frame(FrameType::ThinUpBox);
        header2.set_color(header_color);

        let mut header3 = Frame::default().with_label("HEX");
        header3.set_frame(FrameType::ThinUpBox);
        header3.set_color(header_color);

        header_flex.end();
        header_flex.recalc();
    }

    ui.flex_echo_html_color.end();
    ui.flex_echo_html_color.recalc();
}

fn setup_output_handler(o: &mut Output, flex_echo: &mut Flex) {
    o.handle({
        let mut echo_box = flex_echo.clone();
        move |area, evt| {
            if evt == Event::Released {
                app::copy(area.value().as_str());
                echo_box.set_label(format!("已复制到剪贴板: {}", area.value()).as_str());
                true
            } else {
                false
            }
        }
    });
}