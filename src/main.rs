mod spinner;
use crate::spinner as spin;
use crate::spinner::Spinner;
use std::{
    thread,
    time::{Duration, Instant},
};

const BDELIM_ICON: &str = "\u{01f539}";

const REVOLUTIONS: usize = 100;
const WINK: u64 = 1; // ms.

fn cycle(spinner: &mut dyn Spinner, name: &str) {
    let now = Instant::now();

    for i in 0..REVOLUTIONS {
        spinner.message(format!("{}: [{}]", name, i));
        thread::sleep(Duration::from_millis(WINK));
    }
    spinner.stop();
    println!("{}, time elapsed: {}", name, now.elapsed().as_secs_f64());
}

fn main() {
    let mut dummy = spin::DummySpinner::new();
    cycle(&mut dummy, "Dummy");

    thread::sleep(Duration::from_millis(1000));

    let mut daddy = spin::DaddySpinner::new();
    cycle(&mut daddy, "Daddy");

    thread::sleep(Duration::from_millis(1000));

    let mut pretty = spin::PrettySpinner::new();
    cycle(&mut pretty, "Pretty");

    thread::sleep(Duration::from_millis(1000));

    let mut cutie = spin::CutieSpinner::new();
    cycle(&mut cutie, "Cutie");
}
