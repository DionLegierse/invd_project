
extern crate ocl;

use ocl::*;

fn get_total_solutions(board : &Vec<u32>) -> u32{
    let size = (board.len() as f64).sqrt() as usize;

    let mut cummulative_solutions : u32 = 0;

    for x in 0..size {
        for y in 0..size{
            cummulative_solutions += board[y + (x * size)];
        }
    }

    return cummulative_solutions;
}

pub fn knights_tour_opencl(size : usize) -> ocl::Result<u32> {

    let ocl_functions_raw = std::fs::read_to_string("src/ocl/tour.ocl").expect("Error opening file!\n");

    let mut ocl_functions = ("__constant char size = ".to_string()) + 
                        (&(size as i8).to_string()) +
                        (&";\n".to_string()) + 
                        &ocl_functions_raw.to_string();

    ocl_functions = ocl_functions.replace("?", &(size.pow(2)).to_string());

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

    let solutions = get_total_solutions(&result);

    Ok(solutions)
}
