
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
        .dims(8)
        .build()?;

    let result_list : Buffer<i8> = Buffer::builder().len(size.pow(2) * 8).queue(pro_que.queue().clone()).build()?;
    let is_running : Buffer<u8> = Buffer::builder().len(1).queue(pro_que.queue().clone()).fill_val(1u8).build()?;

    let kernel_tour = pro_que.kernel_builder("knights_tour_solve")
        .arg(&result_list)
        .arg(&is_running)
        .arg(&(start as i8))
        .build()?;

    unsafe{ 
        kernel_tour.enq()?;
    }

    let mut results = vec![0i8; result_list.len()];
    let mut result = vec![0i8; size.pow(2)];

    result_list.read(&mut results).enq()?;

    for n in 0..8 {
        let start = size.pow(2) * n;
        let end = start + size.pow(2);
        let slice = &results[start..end];

        if !slice.contains(&0i8) {
            result = slice.to_vec();
        }
    }

    Ok(result)
}
