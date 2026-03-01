use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src = Path::new("knowledge/train.json");
    let dst = Path::new(&out_dir).join("train.json");

    if !src.exists() {
        fs::create_dir_all("knowledge").ok();
        fs::write(src, "[]").expect("Failed to create default train.json");
    }

    fs::copy(src, dst).expect("Failed to copy train.json to OUT_DIR");
}
