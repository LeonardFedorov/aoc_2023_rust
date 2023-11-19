//Module to hold day dispatching logic to keep separate from the input validation and processing

mod day01;

pub fn run_day(day: i32, input_data: String) -> (String, String) {

    match day {
        1 => day01::day1_solution(input_data),
        _ => ("Not Implemented".to_string(), "Not Implemented".to_string()),
    }

}

