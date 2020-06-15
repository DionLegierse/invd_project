pub enum PLATFORM {
    CPU,
    GPU,
    BOTH
}

pub struct Tour_Args {
    pub platform: PLATFORM,
    pub start: usize,
    pub size: usize
}

pub fn parse_arguments_tour(args: &Vec<String>) -> Result<Tour_Args, &'static str>{
    let mut tour_args = Tour_Args{
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
            let param = &args[index + 1];
            if param.len() != 2 {
                return Err("Incorrect start position given");
            }

            let chess_position : Vec<char> = param.chars().collect();
            let letter = chess_position[0].to_digit(10).unwrap() as usize;
            let number = chess_position[1].to_digit(10).unwrap() as usize;

            tour_args.start = number + (letter * tour_args.size);
        }
    }

    Ok(tour_args)
}