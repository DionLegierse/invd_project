
extern crate ocl;

use ocl::*;

fn print_ocl_board(board : &Vec<u32>){
    let size = (board.len() as f64).sqrt() as usize;

    let mut cummulative_solutions : u32 = 0;

    println!("Solutions per starting position:\n");

    for x in 0..size {
        for y in 0..size{
            print!("[{}]\t", board[y + (x * size)]);
            cummulative_solutions += board[y + (x * size)];
        }
        print!("\n");
    }
    print!("\n");

    println!("Total amount of solutions: {}", cummulative_solutions);
}

pub fn knights_tour_opencl(size : usize) -> ocl::Result<()> {

    let ocl_functions_raw = std::fs::read_to_string("src/ocl/tour.ocl").expect("Error opening file!\n");

    let ocl_functions = ("__constant char size = ".to_string()) + 
                        (&(size as i8).to_string()) +
                        (&";\n".to_string()) + 
                        &ocl_functions_raw.to_string();

    let pro_que = ProQue::builder()
        .src(ocl_functions)
        .dims(size.pow(2))
        .build()?;

    let result_list = pro_que.create_buffer::<u32>()?;

    let kernel_tour = pro_que.kernel_builder("knights_tour_solve")
        .arg(&result_list)
        .build()?;

    unsafe{ 
        kernel_tour.enq()?;
    }

    let mut result = vec![0u32; result_list.len()];
    result_list.read(&mut result).enq()?;

    print_ocl_board(&result);

    Ok(())
}
