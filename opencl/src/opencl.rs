fn trivial() -> ocl::Result<()> {

    let ocl_function = std::fs::read_to_string("src/functions.ocl").expect("Error opening file!\n");

    let pro_que = ProQue::builder()
        .src(ocl_function.to_string())
        .dims(1 << 28)
        .build()?;

    let buffer_a = pro_que.create_buffer::<i32>()?;
    let buffer_b = pro_que.create_buffer::<i32>()?;
    let result = pro_que.create_buffer::<i32>()?;

    let kernel_a = pro_que.kernel_builder("init_vector")
        .arg(&buffer_a)
        .arg(10i32)
        .build()?;

    let kernel_b = pro_que.kernel_builder("init_vector")
        .arg(&buffer_b)
        .arg(100i32)
        .build()?;

    unsafe{ 
        kernel_a.enq()?;
        kernel_b.enq()?;
    }

    let kernel_result = pro_que.kernel_builder("vector_add")
        .arg(&buffer_a)
        .arg(&buffer_b)
        .arg(&result)
        .build()?;

    unsafe{
        kernel_result.enq()?;
    }

    let mut vec = vec![0i32; result.len()];
    result.read(&mut vec).enq()?;

    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);
    Ok(())
}
