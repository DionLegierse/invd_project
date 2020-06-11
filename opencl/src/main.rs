extern crate ocl;
use ocl::ProQue;

mod tour;
use tour::*;

use std::io::{self, Read};

fn main() {
    let mut tour = Tour::new(10);
    let mut nodes : Vec<Vec<usize>> = Vec::new();

    nodes.push(tour.get_move_list());

    loop{
        if tour.is_solved() {
            println!("Solved!");
            tour.print_board();
            break;
        }

        let node_option = nodes.pop();
        let mut node;

        match node_option {
            None => break,
            _ => node = node_option.unwrap()
        }

        if node.len() > 0 {
            let next_move = node.pop().unwrap();
            nodes.push(node);

            tour.set_move(next_move);
            nodes.push(tour.get_move_list());
        }else{
            tour.move_back();
        }
    }

    println!("Hello, world!");
}
