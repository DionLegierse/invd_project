pub enum PLATFORM {
    CPU,
    GPU,
    BOTH
}

pub struct TourArgs {
    pub platform: PLATFORM,
    pub start: usize,
    pub size: usize
}

pub fn parse_arguments_tour(args: &Vec<String>) -> Result<TourArgs, &'static str>{
    let mut tour_args = TourArgs{
        platform: PLATFORM::CPU,
        start : 0,
        size : 0
    };

    for (index, arg) in (&args).iter().enumerate(){
        if arg == "-t" {
            match &args[index + 1][..] {
                "c" => tour_args.platform = PLATFORM::CPU,
                "b" => tour_args.platform = PLATFORM::BOTH,
                "g" => tour_args.platform = PLATFORM::GPU,
                _ => return Err("Incorrect platform given for -t")
            }
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

    Ok(tour_args)
}