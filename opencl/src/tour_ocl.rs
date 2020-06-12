
extern crate ocl;

use ocl::*;

fn print_ocl_board(board : &Vec<i8>){
    let size = (board.len() as f64).sqrt() as usize;

    for x in 0..size {
        for y in 0..size{
            print!("[{}]\t", board[y + (x * size)]);
        }
        print!("\n");
    }
    print!("\n");
}

pub fn knights_tour_opencl(size : usize) -> ocl::Result<()> {

    let ocl_function = std::fs::read_to_string("src/tour.ocl").expect("Error opening file!\n");

    let pro_que = ProQue::builder()
        .src(ocl_function.to_string())
        .dims(size.pow(2))
        .build()?;

    let board = pro_que.create_buffer::<i8>()?;

    let char_size = size as i8;

    let kernel_tour = pro_que.kernel_builder("knights_tour_solve")
        .arg(&board)
        .arg(12i8)
        .arg(&char_size)
        .build()?;



    unsafe{ 
        kernel_tour.enq()?;
    }


  

    let mut result = vec![0i8; board.len()];
    board.read(&mut result).enq()?;

    print_ocl_board(&result);

    Ok(())
}
