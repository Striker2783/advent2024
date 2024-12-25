use std::{path::Path, process::exit};

pub mod eight;
pub mod eighteen;
pub mod eleven;
pub mod fifteen;
pub mod five;
pub mod four;
pub mod fourteen;
pub mod nine;
pub mod nineteen;
pub mod one;
pub mod seven;
pub mod seventeen;
pub mod six;
pub mod sixteen;
pub mod ten;
pub mod thirteen;
pub mod three;
pub mod twelve;
mod twenty;
mod twenty_one;
pub mod two;
mod twenty_two;
mod twenty_three;
mod twenty_four;
mod twenty_five;

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
    thirteen::run,
    fourteen::run,
    fifteen::run,
    sixteen::run,
    seventeen::run,
    eighteen::run,
    nineteen::run,
    twenty::run,
    twenty_one::run,
    twenty_two::run,
    twenty_three::run,
    twenty_four::run,
    twenty_five::run,
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
