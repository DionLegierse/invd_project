
extern crate ocl;

use ocl::*;

//Solves the knights tour problem for the given size and start potision using OpenCL
//Return a result containing either a solution, partial solution or an error
pub fn knights_tour_opencl(size : usize, start : usize) -> ocl::Result<Vec<i8>> {
    let dims = 8;

    //Load the OpenCL functions from the ocl file
    let ocl_functions_raw = std::fs::read_to_string("src/ocl/tour.ocl").expect("Error opening file!\n");

    let mut ocl_functions = ("__constant char size = ".to_string()) + 
                        (&(size as i8).to_string()) +
                        (&";\n".to_string()) + 
                        &ocl_functions_raw.to_string();

    //Replace the question mark within the ocl file with the size of the board
    ocl_functions = ocl_functions.replace("?", &(size.pow(2)).to_string());

    //Create the context and give the dimension (amount of work items)
    let pro_que = ProQue::builder()
        .src(ocl_functions)
        .dims(dims)
        .build()?;

    //Create all the required buffers on the GPU for storing and finding the solution
    let result_list : Buffer<i8> =      Buffer::builder()
                                            .len(size.pow(2) * dims)
                                            .queue(pro_que.queue().clone())
                                            .build()?;

    let is_running : Buffer<u8> =       Buffer::builder()
                                            .len(1)
                                            .queue(pro_que.queue().clone())
                                            .fill_val(1u8)
                                            .build()?;

    let solution_id : Buffer<u32> =     Buffer::builder()
                                            .len(1)
                                            .queue(pro_que.queue().clone())
                                            .fill_val(0u32)
                                            .build()?;


    //Create the kernel to find the a solution
    let kernel_tour = pro_que.kernel_builder("knights_tour_solve")
        .arg(&result_list)
        .arg(&is_running)
        .arg(&solution_id)
        .arg(&(start as i8))
        .build()?;

    //Start the kernel on the GPU
    unsafe{ 
        kernel_tour.enq()?;
    }

    //Create the local variables to store and find the solution
    let mut results             = vec![0i8; result_list.len()];
    let mut solution_id_read    = vec![0u32; solution_id.len()];
    let mut result              = vec![];

    //Read the solution of all work items and the id with a working work item
    solution_id.read(&mut solution_id_read).enq()?;
    result_list.read(&mut results).enq()?;

    //Find the correct solution using the id of the workitem that found it
    let start = size.pow(2) * (solution_id_read[0] as usize);
    let end = start + size.pow(2);
    let slice = &results[start..end];

    //Convert the slice of the all results to its own vector
    if !slice.contains(&0i8) {
        result = slice.to_vec();
    }

    //Return the board
    Ok(result)
}
