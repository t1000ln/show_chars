use std::fmt::format;
use fltk::app;
use fltk::enums::{Color, Font};
use fltk::prelude::{GroupExt, TableExt, WidgetExt};
use fltk_table::{SmartTable, TableOpts};
use crate::ui_loader::UserInterface;

pub static CELL_FONT_SIZE: i32 = 14;

pub fn init_ascii_tab(ui: &mut UserInterface) {
    ui.flex_ascii_table.begin();
    let mut table = SmartTable::default()
        .with_size(1, 1)
        .center_of_parent()
        .with_opts(TableOpts {
            rows: 128,
            cols: 6,
            editable: false,
            cell_font: Font::Screen,
            cell_font_size: CELL_FONT_SIZE,
            cell_border_color: Color::by_index(52),
            header_color: Color::by_index(54),
            ..Default::default()
        });
    table.set_color(ui.flex_ascii_table.color());

    table.set_col_header_value(0, "Bin\n(二进制)");
    table.set_col_header_value(1, "Oct\n(八进制)");
    table.set_col_header_value(2, "Dec\n(十进制)");
    table.set_col_header_value(3, "Hex\n(十六进制)");
    table.set_col_header_value(4, "缩写 / 字符");
    table.set_col_header_value(5, "解释");

    table.set_col_header_height(40);
    table.set_row_height_all(32);
    table.set_col_width(0, 18 / 2 * 12);
    // table.set_col_width(0, 18 / 2 * 10);
    // table.set_col_width(0, 18 / 2 * 10);
    // table.set_col_width(0, 18 / 2 * 10);
    table.set_col_width(4, 18 / 2 * 26);
    table.set_col_width(5, 18 / 2 * 13);

    table.set_callback({
        let mut echo_box = ui.box_echo_ascii.clone();
        move |st| {
            // println!("{:?}", st.get_selection());
            let (r1, c1, _, _) = st.get_selection();
            let v = st.cell_value(r1, c1);
            app::copy(v.as_str());
            echo_box.set_label(format!("已复制到剪贴板:\n {}", v).as_str());
        }
    });

    ui.flex_ascii_table.end();

    fill_table(&mut table);
}

fn fill_table(table: &mut SmartTable) {
    auto_fill_math_value(table);

    fill_static_desc(table);
}

fn auto_fill_math_value(table: &mut SmartTable) {
    for r in 0..table.rows() {
        table.set_cell_value(r, 0, format!("{r:#010b}").as_str());
        table.set_cell_value(r, 1, format!("{r:#04o}").as_str());
        table.set_cell_value(r, 2, format!("{r}").as_str());
        table.set_cell_value(r, 3, format!("{r:#04X}").as_str());
    }

}

fn fill_static_desc(table: &mut SmartTable) {

    table.set_cell_value(0, 5, "空字符");
    for (row, (c, d)) in ASCII_DESC.iter().enumerate() {
        table.set_cell_value(row as i32, 4, c);
        table.set_cell_value(row as i32, 5, d);
    }
}


pub const ASCII_DESC: &[(&str, &str)] = &[
    ("NUL(null)", "空字符"),
    ("SOH(start of headline)", "标题开始"),
    ("STX (start of text)", "正文开始"),
    ("ETX (end of text)", "正文结束"),
    ("EOT (end of transmission)", "传输结束"),
    ("ENQ (enquiry)", "请求"),
    ("ACK (acknowledge)", "收到通知"),
    ("BEL (bell)", "响铃"),
    ("BS (backspace)", "退格"),
    ("HT (horizontal tab)", "水平制表符"),
    ("LF (NL line feed, new line)", "换行键"),
    ("VT (vertical tab)", "垂直制表符"),
    ("FF (NP form feed, new page)", "换页键"),
    ("CR (carriage return)", "回车键"),
    ("SO (shift out)", "不用切换"),
    ("SI (shift in)", "启用切换"),
    ("DLE (data link escape)", "数据链路转义"),
    ("DC1 (device control 1)", "设备控制1"),
    ("DC2 (device control 2)", "设备控制2"),
    ("DC3 (device control 3)", "设备控制3"),
    ("DC4 (device control 4)", "设备控制4"),
    ("NAK (negative acknowledge)", "拒绝接收"),
    ("SYN (synchronous idle)", "同步空闲"),
    ("ETB (end of trans. block)", "结束传输块"),
    ("CAN (cancel)", "取消"),
    ("EM (end of medium)", "媒介结束"),
    ("SUB (substitute)", "代替"),
    ("ESC (escape)", "换码(溢出)"),
    ("FS (file separator)", "文件分隔符"),
    ("GS (group separator)", "分组符"),
    ("RS (record separator)", "记录分隔符"),
    ("US (unit separator)", "单元分隔符"),
    ("(space)", "空格"),
    ("!", "叹号"),
    ("\"", "双引号"),
    ("#", "井号"),
    ("$", "美元符"),
    ("%", "百分号"),
    ("&", "和号"),
    ("'", "闭单引号"),
    ("(", "开括号"),
    (")", "闭括号"),
    ("*", "星号"),
    ("+", "加号"),
    (",", "逗号"),
    ("-", "减号/破折号"),
    (".", "句号"),
    ("/", "斜杠"),
    ("0", "字符0"),
    ("1", "字符1"),
    ("2", "字符2"),
    ("3", "字符3"),
    ("4", "字符4"),
    ("5", "字符5"),
    ("6", "字符6"),
    ("7", "字符7"),
    ("8", "字符8"),
    ("9", "字符9"),
    (":", "冒号"),
    (";", "分号"),
    ("<", "小于"),
    ("=", "等号"),
    (">", "大于"),
    ("?", "问号"),
    ("@", "电子邮件符号"),
    ("A", "大写字母A"),
    ("B", "大写字母B"),
    ("C", "大写字母C"),
    ("D", "大写字母D"),
    ("E", "大写字母E"),
    ("F", "大写字母F"),
    ("G", "大写字母G"),
    ("H", "大写字母H"),
    ("I", "大写字母I"),
    ("J", "大写字母J"),
    ("K", "大写字母K"),
    ("L", "大写字母L"),
    ("M", "大写字母M"),
    ("N", "大写字母N"),
    ("O", "大写字母O"),
    ("P", "大写字母P"),
    ("Q", "大写字母Q"),
    ("R", "大写字母R"),
    ("S", "大写字母S"),
    ("T", "大写字母T"),
    ("U", "大写字母U"),
    ("V", "大写字母V"),
    ("W", "大写字母W"),
    ("X", "大写字母X"),
    ("Y", "大写字母Y"),
    ("Z", "大写字母Z"),
    ("[", "开方括号"),
    ("\\", "反斜杠"),
    ("]", "闭方括号"),
    ("^", "脱字符"),
    ("_", "下划线"),
    ("`", "开单引号"),
    ("a", "小写字母a"),
    ("b", "小写字母b"),
    ("c", "小写字母c"),
    ("d", "小写字母d"),
    ("e", "小写字母e"),
    ("f", "小写字母f"),
    ("g", "小写字母g"),
    ("h", "小写字母h"),
    ("i", "小写字母i"),
    ("j", "小写字母j"),
    ("k", "小写字母k"),
    ("l", "小写字母l"),
    ("m", "小写字母m"),
    ("n", "小写字母n"),
    ("o", "小写字母o"),
    ("p", "小写字母p"),
    ("q", "小写字母q"),
    ("r", "小写字母r"),
    ("s", "小写字母s"),
    ("t", "小写字母t"),
    ("u", "小写字母u"),
    ("v", "小写字母v"),
    ("w", "小写字母w"),
    ("x", "小写字母x"),
    ("y", "小写字母y"),
    ("z", "小写字母z"),
    ("{", "开花括号"),
    ("|", "垂线"),
    ("}", "闭花括号"),
    ("~", "波浪号"),
    ("DEL (delete)", "删除"),
];