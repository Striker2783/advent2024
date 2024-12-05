use std::{path::Path, process::exit};

pub mod five;
pub mod four;
pub mod one;
pub mod three;
pub mod two;

const FNS: &[fn(&Path)] = &[one::run, two::run, three::run, four::run, five::run];

pub fn run(n: u32, f: &Path) {
    let i = (n - 1) as usize;
    if i >= FNS.len() {
        eprintln!("Invalid day");
        exit(0);
    } else {
        FNS[i](f);
    }
}
