use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=ui/chars.fl");
    let g = fl2rust::Generator::default();

    g.in_out("ui/chars.fl", out_path.join("auto_gen_ui.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
    //
    // fs::copy("res/DIGITAL-Dream.ttf", env::temp_dir().join("DIGITAL-Dream.ttf")).expect("Failed to copy font file!");
    // fs::copy("res/DIGITAL-Dream.ttf", env::temp_dir().join("DIGITAL-Dreamfat.ttf")).expect("Failed to copy font file!");
    // fs::copy("res/DIGITAL-Dream.ttf", env::temp_dir().join("DIGITAL-Regular.ttf")).expect("Failed to copy font file!");


}