use fltk::app;
use fltk::enums::{Align, Color, Event, FrameType, LabelType};
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use crate::ui_loader::UserInterface;

pub fn init_fltk_color(ui: &mut UserInterface) {
    ui.flex_fltk_color.begin();

    for i in 0..=255 {
        let rid = i % 8;
        if rid == 0 {
            let mut flex = Flex::default().with_align(Align::Inside | Align::Left);
            flex.set_label_type(LabelType::Normal);
            flex.set_margin(2);
            flex.set_pad(1);
            flex.set_frame(FrameType::FlatBox);
            flex.set_color(Color::by_index(52));

            for j in 0..8 {
                let mut rect = Frame::default().with_label((i + j).to_string().as_str());
                rect.set_label_size(10);
                rect.set_label_type(LabelType::Normal);
                rect.set_frame(FrameType::FlatBox);
                let c = Color::by_index(i + j);
                let (r, g, b) = c.to_rgb();
                if (r as u16 + g as u16 + b as u16) > 508 {
                    rect.set_label_color(Color::Black);
                } else {
                    rect.set_label_color(Color::White);
                }
                rect.set_color(Color::by_index(i + j));
            }

            flex.end();
        }
    }


    ui.flex_fltk_color.end();
    ui.flex_fltk_color.recalc();

    ui.flex_fltk_color.handle({
        let mut echo_box = ui.box_echo_fltk_color_seq.clone();
        move |area, evt| {
            if evt == Event::Released {
                if let Some(w) = app::belowmouse::<Frame>() {
                    // println!("{}", w.label());
                    app::copy(w.label().as_str());
                    echo_box.set_label(format!("已复制到剪贴板: {}", w.label()).as_str());
                }
                true
            } else {
                false
            }
        }
    })
}

// fn setup_btn(ui: &mut UserInterface) {
//     ui.btn_toggle_color_index.set_callback({
//         let mut flex = ui.flex_fltk_color.clone();
//         move |btn| {
//             let toggle = btn.is_on();
//             for i in 0..flex.children() {
//                 if let Some(sub_flex) = flex.child(i) {
//                     if let Some(sub_grp) = sub_flex.as_group() {
//                         for j in 0..sub_grp.children() {
//                             if let Some(mut rect) = sub_grp.child(j) {
//                                 rect.set_label_type(if toggle {LabelType::Normal} else {LabelType::None});
//                             }
//                         }
//                     }
//                 }
//             }
//             flex.set_damage(true);
//         }
//     });
// }