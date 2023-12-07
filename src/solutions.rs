//Module to hold day dispatching logic to keep separate from the input validation and processing

mod day01; 
mod day02; 
mod day03; 
mod day04; 
mod day05; 
//mod day06; 
mod day07; 
//mod day08; 
//mod day09; 
//mod day10;
//mod day11; 
//mod day12; 
//mod day13; 
//mod day14; 
//mod day15; 
//mod day16; 
//mod day17; 
//mod day18; 
//mod day19; 
//mod day20;
//mod day21; 
//mod day22; 
//mod day23; 
//mod day24; 
//mod day25;

pub fn run_day(day: i32, input_data: String) -> (String, String) {

    match day {
        01 => day01::day01_solution(input_data),
        02 => day02::day02_solution(input_data),
        03 => day03::day03_solution(input_data),
        04 => day04::day04_solution(input_data),
        05 => day05::day05_solution(input_data),
        //06 => day06::day06_solution(input_data),
        07 => day07::day07_solution(input_data),
        //08 => day08::day08_solution(input_data),
        //09 => day09::day09_solution(input_data),
        //10 => day10::day10_solution(input_data),
        //11 => day11::day11_solution(input_data),
        //12 => day12::day12_solution(input_data),
        //13 => day13::day13_solution(input_data),
        //14 => day14::day14_solution(input_data),
        //15 => day15::day15_solution(input_data),
        //16 => day16::day16_solution(input_data),
        //17 => day17::day17_solution(input_data),
        //18 => day18::day18_solution(input_data),
        //10 => day19::day19_solution(input_data),
        //20 => day20::day20_solution(input_data),
        //21 => day21::day21_solution(input_data),
        //22 => day22::day22_solution(input_data),
        //23 => day23::day23_solution(input_data),
        //24 => day24::day24_solution(input_data),
        //25 => day25::day25_solution(input_data),
         _ => ("Not Implemented".to_string(), "Not Implemented".to_string()),
    }

}

