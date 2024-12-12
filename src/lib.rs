use std::{path::Path, process::exit};

pub mod eight;
pub mod eleven;
pub mod five;
pub mod four;
pub mod nine;
pub mod one;
pub mod seven;
pub mod six;
pub mod ten;
pub mod three;
pub mod twelve;
pub mod two;

const FNS: &[fn(&Path)] = &[
    one::run,
    two::run,
    three::run,
    four::run,
    five::run,
    six::run,
    seven::run,
    eight::run,
    nine::run,
    ten::run,
    eleven::run,
    twelve::run,
];

pub fn run(n: u32, f: &Path) {
    let i = (n - 1) as usize;
    if i >= FNS.len() {
        eprintln!("Invalid day");
        exit(0);
    } else {
        FNS[i](f);
    }
}
