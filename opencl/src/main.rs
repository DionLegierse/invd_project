use std::time::{Instant};

extern crate ocl;

mod tour;
use tour::*;

mod tour_ocl;
use tour_ocl::knights_tour_opencl;

mod tour_args;
use tour_args::*;

use std::env;

fn run_cpu(size: usize, start: usize){
    let now = Instant::now();
    let mut tour = Tour::new(start, size);
    let board = tour.solve();
    let solution_time_ms = now.elapsed().as_millis();

    for x in 0..size {
        for y in 0..size {
            let pos = x + (y * size);
            print!("[{}]\t", board[pos]);
        }
        print!("\n");
    }

    println!("{}.{} seconden over deze oplossing via cpu", solution_time_ms / 1000,solution_time_ms % 1000);
}

fn run_gpu(size: usize, start: usize){
        let now = Instant::now();
        let result = knights_tour_opencl(size, size.pow(2)).unwrap();
        let end = now.elapsed();
        println!("{}.{} seconden over gedaan over {} oplossingen via gpu", end.as_secs(), now.elapsed().as_millis() % 1000, result);
}

fn run_both(size: usize, start: usize){
    run_cpu(size, start);
    run_gpu(size, start);
    println!("Running both");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = parse_arguments_tour(&args).unwrap();

    match parsed_args.platform {
        PLATFORM::BOTH => run_both(parsed_args.size, parsed_args.start),
        PLATFORM::CPU => run_cpu(parsed_args.size, parsed_args.start),
        PLATFORM::GPU => run_gpu(parsed_args.size, parsed_args.start)
    }
}
