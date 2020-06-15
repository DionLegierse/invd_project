
extern crate ocl;

use ocl::*;

pub fn knights_tour_opencl(size : usize, start : usize) -> ocl::Result<Vec<i8>> {

    let ocl_functions_raw = std::fs::read_to_string("src/ocl/tour.ocl").expect("Error opening file!\n");

    let mut ocl_functions = ("__constant char size = ".to_string()) + 
                        (&(size as i8).to_string()) +
                        (&";\n".to_string()) + 
                        &ocl_functions_raw.to_string();

    ocl_functions = ocl_functions.replace("?", &(size.pow(2)).to_string());

    let pro_que = ProQue::builder()
        .src(ocl_functions)
        .dims(1)
        .build()?;

    let result_list : Buffer<i8> = Buffer::builder().len(size.pow(2)).queue(pro_que.queue().clone()).build()?;

    let kernel_tour = pro_que.kernel_builder("knights_tour_solve")
        .arg(&result_list)
        .arg(&(start as i8))
        .build()?;

    unsafe{ 
        kernel_tour.enq()?;
    }

    let mut result = vec![0i8; result_list.len()];
    result_list.read(&mut result).enq()?;

    Ok(result)
}
