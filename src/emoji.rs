use fltk::app;
use fltk::enums::{Color, Font};
use fltk::prelude::{GroupExt, InputExt, TableExt, WidgetExt};
use fltk_table::{SmartTable, TableOpts};
use crate::ui_loader::UserInterface;

pub const EMOJI_COLS: i32 = 16;
pub const EMOJI_FONT_SIZE: i32 = 20;

pub const EMOJI_CHAR_BYTES: &[u8] = include_bytes!("../resource/emoji2.txt");
// pub const EMOJI_CHAR_STR: &str = include_str!("../resource/emoji2.txt");

pub fn init_emoji(ui: &mut UserInterface) {
    let char_string = String::from_utf8_lossy(EMOJI_CHAR_BYTES).to_string();

    ui.out_hanzi_reg.set_value("[\\u3400-\\u4DBF\\u4E00-\\u9FFF\\uF900-\\uFAFF] 或 \\p{Han}");

    ui.flex_emoji_table.begin();

    let max_rows = char_string.chars().filter(|c| *c as u32 != 0xFE0F).count() as i32 / EMOJI_COLS + 1;
    let mut table = SmartTable::default()
        .with_opts(TableOpts {
            rows: max_rows,
            cols: EMOJI_COLS,
            editable: false,
            cell_font: Font::Symbol,
            cell_font_size: EMOJI_FONT_SIZE,
            cell_border_color: Color::by_index(52),
            header_color: Color::by_index(52),
            ..Default::default()
        });
    table.set_col_width_all(64);
    table.set_row_height_all(64);
    table.set_col_header(false);
    table.set_row_header(false);

    let (mut col, mut row) = (0, 0);

    let mut skip_next_char = false;
    for (idx, c) in char_string.chars().enumerate() {
        // println!("{} {}", c, c as u32); out_hanzi_reg.set_value("[\u3400-\u4DBF\u4E00-\u9FFF\uF900-\uFAFF]\p{Han}");
        if skip_next_char {
            skip_next_char = false;
            continue;
        } else {
            let cs = if char_string.chars().nth(idx + 1).is_some_and(|nc| nc as u32 == 0xFE0F) {
                skip_next_char = true;
                format!("{}{}", c, "\u{fe0f}")
            } else {
                skip_next_char = false;
                c.to_string()
            };

            if col > 0 && col % EMOJI_COLS == 0 {
                row += 1;
                col = 0;
            }
            table.set_cell_value(row, col, cs.as_str());
            col += 1;
        }


    }
    table.end();

    table.set_callback({
        let mut echo_box = ui.box_echo_emoji.clone();
        move |st| {
            // println!("{:?}", st.get_selection());
            let (r1, c1, _, _) = st.get_selection();
            if r1 >= 0 && c1 >= 0 {
                let v = st.cell_value(r1, c1);
                let decoded = v.chars().map(|c| format!("{:X?}", c as u32)).collect::<Vec<_>>().join("  ");
                app::copy(v.as_str());
                echo_box.set_label(format!("已复制到剪贴板: {} => {}", v, decoded).as_str());
            }

        }
    });

    ui.flex_emoji_table.end();
    ui.flex_emoji_table.recalc();
}


// pub const EMOJI_CHAR_CODES: &[&str] = &[
//     "\u{00a9}", "\u{00ae}", "\u{203c}", "\u{2049}", "\u{2122}", "\u{2139}", "\u{2194}", "\u{2195}", "\u{2196}", "\u{2197}",
//     "\u{2198}", "\u{2199}", "\u{21a9}", "\u{21aa}", "\u{231a}", "\u{231b}", "\u{2328}", "\u{23cf}", "\u{23e9}", "\u{23ea}",
//     "\u{23eb}", "\u{23ec}", "\u{23ed}", "\u{23ee}", "\u{23ef}", "\u{23f0}", "\u{23f1}", "\u{23f2}", "\u{23f3}", "\u{23f8}",
//     "\u{23f9}", "\u{23fa}", "\u{23fb}", "\u{24c2}", "\u{25aa}", "\u{25ab}", "\u{25b6}", "\u{25c0}", "\u{25fb}", "\u{25fc}",
//     "\u{25fd}", "\u{25fe}", "\u{2600}", "\u{2601}", "\u{2602}", "\u{2603}", "\u{2604}", "\u{260e}", "\u{2611}", "\u{2614}",
//     "\u{2615}", "\u{2618}", "\u{261d}", "\u{2620}", "\u{2622}", "\u{2623}", "\u{2626}", "\u{262a}", "\u{262e}", "\u{262f}",
//     "\u{2638}", "\u{2639}", "\u{263a}", "\u{2640}", "\u{2642}", "\u{2648}", "\u{2649}", "\u{264a}", "\u{264b}", "\u{264c}",
//     "\u{264d}", "\u{264e}", "\u{264f}", "\u{2650}", "\u{2651}", "\u{2652}", "\u{2653}", "\u{265f}", "\u{2660}", "\u{2663}",
//     "\u{2665}", "\u{2666}", "\u{2668}", "\u{267b}", "\u{267e}", "\u{267f}", "\u{2692}", "\u{2693}", "\u{2694}", "\u{2695}",
//     "\u{2696}", "\u{2697}", "\u{2699}", "\u{269b}", "\u{269c}", "\u{26a0}", "\u{26a1}", "\u{26a7}", "\u{26aa}", "\u{26ab}",
//     "\u{26b0}", "\u{26b1}", "\u{26bd}", "\u{26be}", "\u{26c4}", "\u{26c5}", "\u{26c8}", "\u{26ce}", "\u{26cf}", "\u{26d1}",
//     "\u{26d3}", "\u{26d4}",
// ];