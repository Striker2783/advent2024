use std::{path::Path, process::exit};

pub mod one;
pub mod two;
pub mod three;
pub mod four;

pub fn run(n: u32, f: &Path) {
    match n {
        1 => one::run(f),
        2 => two::run(f),
        3 => three::run(f),
        4 => four::run(f),
        _ => {
            eprintln!("Invalid day");
            exit(0);
        }
    }
}
