use std::env;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_outcome = run_day(args);

    match day_outcome {
        Ok(()) => (),
        Err(e) => println!("Error occured: {e}"),
    }
}

//Main function - returns an error if something goes wrong
fn run_day(args: Vec<String>) -> Result<(), &'static str> {

    if args.len() < 2 {
        return Err("No arguments supplied");
    }

    //Attempt to parse argument 1 as the day number
    let day_num = i32::from_str(&args[1]);
    //Return fail if the day number cannot be parsed as an integer
    match day_num {
        Err(_e) => return Err("Argument 1 was not a valid integer"),
        Ok(_n) => (),
    }
    //Unwrap day number as it is now known to be safe and test if it is in bounds
    let day_num = day_num.unwrap();
    if day_num < 1 || day_num > 25 {
        return Err("Day number out of bounds");
    }

    //If a second argument is present, then swap to load test data
    let load_test_data = args.len() > 2;

    let file_path = if load_test_data {
        format!("input_data\\test\\Day{day_num}Test.txt")
    } else {
        format!("input_data\\Day{day_num}Input.txt")
    };

    let file_data = fs::read_to_string(&file_path);

    match &file_data {
        Err(_e) => {
            //Force leak the error string so that the reference can propogate upwards
            let static_error_msg = Box::leak(format!("Could not read file from path: {}", &file_path).into_boxed_str());
            return Err(static_error_msg);
        },
        Ok(_s) => (),
    }

    let file_data = file_data.unwrap();

    let before = Instant::now();
    let (part1_result, part2_result) = solutions::run_day(day_num, file_data);
    let after = before.elapsed();

    println!("Part 1: {}\nPart 2: {}\nTime taken: {}ms", part1_result, part2_result, after.as_millis());

    return Ok(());
}