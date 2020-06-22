use std::time::{Instant};
use std::env;
use std::fs::File;
use std::io::Write;

extern crate ocl;

mod tour;
use tour::*;

mod tour_ocl;
use tour_ocl::knights_tour_opencl;

mod tour_args;
use tour_args::*;

fn save_solution(name : String, board : &Vec<i8>, size : usize){

    if board.len() == 0 {
        return;
    }

    let mut html = std::fs::read_to_string("src/chessboard.html").expect("Error opening chessboard.html!\n");

    html.push_str("<table>\n");

    for x in 0..size {
        html.push_str("<tr>\n");

        for y in 0..size {
            let pos = board[x + (y * size)].to_string();
            
            if ((x + y) % 2) == 1{
                html.push_str(r#"<td>"#);
            }else{
                html.push_str(r#"<td style="background-color:grey">"#);
            }
            
            html.push_str(&pos);
            html.push_str("</td>\n");
        }

        html.push_str("<tr>\n");
    }

    html.push_str("</table>");

    let mut save_file = File::create(name + &String::from(".html")).expect("Error making .html!\n");
    save_file.write_all(html.as_bytes()).unwrap();
}

fn print_board(board : &Vec<i8>, size : usize){
    if board.len() == 0 || board.contains(&0) || board.contains(&-1i8){
        println!("Geen oplossing gevonden");
        return;
    }

    for x in 0..size {
        for y in 0..size {
            let pos = y + (x * size);
            print!("[{}]\t", board[pos]);
        }
        print!("\n");
    }
}

fn run_cpu(size: usize, start: usize){
    let now = Instant::now();
    let mut tour = Tour::new(start, size);
    let board = tour.solve();
    let solution_time_ms = now.elapsed().as_millis();

    print_board(&board, size);
    save_solution(String::from("cpu_solutions"), &board, size);

    println!("{}.{} seconden over deze oplossing via cpu", solution_time_ms / 1000,solution_time_ms % 1000);
}

fn run_gpu(size: usize, start: usize){
        let now = Instant::now();
        let board = knights_tour_opencl(size, start).unwrap();
        let end = now.elapsed();

        print_board(&board, size);
        save_solution(String::from("gpu_solutions"), &board, size);

        println!("{}.{} seconden over gedaan over oplossingen via gpu", end.as_secs(), now.elapsed().as_millis() % 1000);
}

fn run_both(size: usize, start: usize){
    run_gpu(size, start);
    run_cpu(size, start);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = parse_arguments_tour(&args).unwrap();

    match parsed_args.platform {
        PLATFORM::BOTH => run_both(parsed_args.size, parsed_args.start),
        PLATFORM::CPU => run_cpu(parsed_args.size, parsed_args.start),
        PLATFORM::GPU => run_gpu(parsed_args.size, parsed_args.start),
        _ => println!("Incorrect platform given use -c, -g or -b")
    }
}
