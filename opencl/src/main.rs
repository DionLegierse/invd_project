use std::time::{Instant};

extern crate ocl;

mod tour;
use tour::*;

use std::env;

fn main() {
    let mut solution_amount = 0;
    let mut solution_time_ms = 0;

    let args: Vec<String> = env::args().collect();

    let size : usize = args[1].parse().unwrap();

    for n in 0..size.pow(2){

        let mut tour = Tour::new(n, size);

        let now = Instant::now();
        let result = tour.solve();

        solution_time_ms += now.elapsed().as_millis();
        solution_amount += result.len();
    }

    println!("{} milliseconden over gedaan over {} oplossingen", solution_time_ms, solution_amount);
}
