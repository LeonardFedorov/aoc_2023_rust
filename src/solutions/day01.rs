use std::str::FromStr;

pub fn day1_solution(input_data: String) -> (String, String) {

    //Part 1
    let mut p1_sum = 0;

    let mut first_digit: char = '0';
    let mut last_digit: char = '0';

    for line in input_data.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                first_digit = c;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last_digit = c;
                break;
            }            
        }

    p1_sum += i64::from_str(&format!("{first_digit}{last_digit}")).expect("String parse failed");
    }

    //Part 2
    let mut p2_sum = 0;
    let mut first_digit2 = 0;
    let mut last_digit2 = 0;

    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in input_data.lines() {

        let first_digits = numbers.iter().map(|digit| line.find(digit)).collect::<Vec<Option<usize>>>();
        let last_digits = numbers.iter().map(|digit| line.rfind(digit)).collect::<Vec<Option<usize>>>();

        let mut first_index = first_digits.len() + 1;
        let mut first_digit = 10;

        let mut second_index = 0;
        let mut second_digit = 10;       

        for i in 0 ..= 9 {
            
            if Option::is_some(&first_digits[i]) {
                let target_index = first_digits[i].unwrap();
                if target_index < first_index {
                    first_index = target_index;
                    first_digit = i;
                }
            }

            if Option::is_some(&last_digits[i]) {
                let target_index = last_digits[i].unwrap();
                if target_index > second_index {
                    second_index = target_index;
                    second_digit = i;
                }
            } 

        }

        p2_sum += 10*first_digit + second_digit
    }


    return (p1_sum.to_string(), p2_sum.to_string());
}