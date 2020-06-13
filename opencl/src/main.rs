use std::time::{Instant};

extern crate ocl;

mod tour;
use tour::*;

mod tour_ocl;
use tour_ocl::knights_tour_opencl;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let size : usize = args[2].parse().unwrap();
    let to_run = &args[1];

    if to_run == "cpu" || to_run == "both" {
        let mut solution_amount = 0;
        let mut solution_time_ms = 0;

        for n in 0..size.pow(2){

            let mut tour = Tour::new(n, size);
    
            let now = Instant::now();
            let result = tour.solve();
    
            solution_time_ms += now.elapsed().as_millis();
            solution_amount += result.len();
        }
    
        println!("{}.{} seconden over gedaan over {} oplossingen via cpu", solution_time_ms / 1000,solution_time_ms % 1000, solution_amount);
    }

    if to_run == "gpu" || to_run == "both" {
        let now = Instant::now();
        let result = knights_tour_opencl(size).unwrap();
        let end = now.elapsed();
        println!("{}.{} seconden over gedaan over {} oplossingen via gpu", end.as_secs(), now.elapsed().as_millis() % 1000, result);
    }
}
