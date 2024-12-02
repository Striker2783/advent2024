use std::{path::Path, process::exit};

pub mod one;
pub mod two;

pub fn run(n: u32, f: &Path) {
    match n {
        1 => one::run(f),
        2 => two::run(f),
        _ => {
            eprintln!("Invalid day");
            exit(0);
        }
    }
}
