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
    let leading_zero = if day_num < 10 {"0"} else {""};

    let file_path = if load_test_data {
        format!("input_data\\test\\Day{leading_zero}{day_num}Test.txt")
    } else {
        format!("input_data\\Day{leading_zero}{day_num}Input.txt")
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

    //If the code takes more 2^63 milliseconds to run and breaks the casts, joke's on me I guess
    let millis = after.as_millis() as i64;
    
    let time: String;
    let unit: String;

    if millis < 10 {
        unit = "μ".to_string();
        time = integer_with_commas(after.as_micros() as i64);
    } else if millis < 10_000 {
        unit = "m".to_string();
        time = integer_with_commas(millis);
    } else {
        unit = String::new();
        time = integer_with_commas(after.as_secs() as i64);
    }
   
    println!("\nDay {day_num}\n\nPart 1: {part1_result}\nPart 2: {part2_result}\n\nTime taken: {time} {unit}s\n");

    return Ok(());
}

//Function to insert comma place separators into a printed integer
//Uses accounting bracket notation for negative numbers
pub fn integer_with_commas(number: i64) -> String {
    
    //Convert the number to a string and get basic info
    let is_negative = number < 0;
    let num_string = number.to_string();
    let len = num_string.len() - (if is_negative {1} else {0});

    let mut num_iter = num_string.chars();
    
    //If the number is negative, skip over the minus sign inserted by the string conversion
    if is_negative {num_iter.next();}

    //Worst case length is 3 times - if a negative single digit number is inserted
    let mut new_string = String::with_capacity(3*len);

    if is_negative {new_string.push('(');}

    //Push the last char separately after the loop to prevent the insertion of an extraneous comma
    for i in 0 .. len-1 {
        new_string.push(num_iter.nth(0).expect("String ran out early"));

        //This inserts the commas every third position starting from the right place
        if (i + 1) % 3 == (len % 3) {
            new_string.push(',')
        }
    }
    new_string.push(num_iter.nth(0).expect("String ran out early"));

    if is_negative {new_string.push(')');}

    return new_string;

}