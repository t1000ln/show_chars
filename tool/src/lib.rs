use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorInfo {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
}


pub fn load_colors()  {
    let from_file = File::open("../resource/html_color").unwrap();
    let reader = BufReader::new(from_file);
    let mut origin = serde_json::from_reader::<BufReader<File>, Vec<ColorInfo>>(reader).unwrap();
    // println!("{:#?}", origin);

    // 单号
    let mut odd: Vec<ColorInfo> = Vec::new();
    // 双号
    let mut even: Vec<ColorInfo> = Vec::new();
    let mut total = origin.len();
    while let Some(ci) = origin.pop() {
        if total % 2 == 0 {
            even.push(ci);
        } else {
            odd.push(ci);
        }
        total -= 1;
    }

    even.reverse();
    odd.reverse();
    odd.append(&mut even);

    // println!("{:#?}", odd);

    let to_file = File::create("../resource/html_colors.json").unwrap();
    let writer = BufWriter::new(to_file);
    serde_json::to_writer_pretty(writer, &odd).unwrap();
}

pub fn load_emoji() {
    if let Ok(old) = fs::read_to_string(PathBuf::from("../resource/emoji.txt")) {
        // println!("{old}");

        let new = old.replace("\n", "");
        // println!("{new}");
        let to_file = File::create("../resource/emoji2.txt").unwrap();
        let mut writer = BufWriter::new(to_file);
        writer.write_all(new.as_bytes()).unwrap();
    }

    // let to_file = File::create("../resource/html_colors.json").unwrap();
    // let writer = BufWriter::new(to_file);
    // serde_json::to_writer_pretty(writer, &odd).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test() {
        load_colors();
    }

    #[test]
    fn simple_test() {
        let r = 2 / 3;
        println!("{r}");
        assert_eq!(r, 0);

        println!("\u{a9}");
        println!("\u{a9}\u{fe0f}");
    }

    #[test]
    fn load_emoji_test() {
        load_emoji();
    }
}
