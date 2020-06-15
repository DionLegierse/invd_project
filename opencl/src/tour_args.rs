pub enum PLATFORM {
    CPU,
    GPU,
    BOTH,
    NONE
}

pub struct TourArgs {
    pub platform: PLATFORM,
    pub start: usize,
    pub size: usize
}

pub fn parse_arguments_tour(args: &Vec<String>) -> Result<TourArgs, &'static str>{
    let mut tour_args = TourArgs{
        platform: PLATFORM::NONE,
        start : 0,
        size : 0
    };

    for arg in args {
        match &arg[..] {
            "-c" => tour_args.platform = PLATFORM::CPU,
            "-b" => tour_args.platform = PLATFORM::BOTH,
            "-g" => tour_args.platform = PLATFORM::GPU,
            &_ => ()
        }
    }

    for (index, arg) in (&args).iter().enumerate(){
        if arg == "-s" {
            let size = &args[index + 1].parse::<usize>();

            match size {
                Ok(n) => tour_args.size = *n,
                _ => return Err("Incorrect size given for -s")
            }
        }
    }

    for (index, arg) in (&args).iter().enumerate(){
        if arg == "-p" {
            let size = &args[index + 1].parse::<usize>();

            match size {
                Ok(n) => tour_args.start = *n,
                _ => return Err("Incorrect size given for -s")
            }
        }
    }

    if tour_args.size <= tour_args.start {
        return Err("Start is outside of board");
    }

    Ok(tour_args)
}