mod spinner;
use crate::spinner as spin;
use crate::spinner::Spinner;
use std::{
    cmp,
    ffi::OsStr,
    fs, io,
    io::Write,
    path::{Path, PathBuf},
    process::exit,
    thread,
    time::{Duration, Instant},
};

const BDELIM_ICON: &str = "\u{01f539}";

fn cycle(spinner: &mut dyn Spinner, _n: usize, _wink: usize) {
    spinner.message("Zzz".into());
    thread::sleep(Duration::from_millis(1000));
}

fn main() {
    let now = Instant::now();

    let mut daddy = spin::DaddySpinner::new();
    cycle(&mut daddy, 0, 0);
    daddy.stop();

    thread::sleep(Duration::from_millis(1000));

    let mut cutie = spin::CutieSpinner::new();
    cycle(&mut cutie, 0, 0);
    cutie.stop();

    thread::sleep(Duration::from_millis(1000));

    println!("Hello, world!");
}
