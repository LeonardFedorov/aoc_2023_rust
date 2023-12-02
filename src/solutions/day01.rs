pub fn day01_solution(input_data: String) -> (String, String) {

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    //Process each line in term, updating the running totals in the sum variables
    for line in input_data.lines() {

        let mut first_digit = 0;
        let mut last_digit = 0;

        let mut first_index: usize = 0;
        let mut last_index: usize = line.len() - 1;

        //First, find first and last digits to do part 1
        for c in line.chars() {
            if c.is_digit(10) {
                first_digit = (c as i64) - 0x30;
                break;
            }
            first_index += 1;
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last_digit = (c as i64) - 0x30;
                break;
            }
            last_index -= 1;            
        }
        p1_sum += 10*first_digit + last_digit;

        //Next, search for words for part 2
        let first_digits = numbers.iter().map(|digit| line.find(digit)).collect::<Vec<Option<usize>>>();
        let last_digits = numbers.iter().map(|digit| line.rfind(digit)).collect::<Vec<Option<usize>>>();

        for i in 0 .. numbers.len() {
            
            //Search for whether there are any words that occur before the first digit identified
            //and if so, find the earliest of them
            if Option::is_some(&first_digits[i]) {
                let target_index = first_digits[i].unwrap();
                if target_index < first_index {
                    first_index = target_index;
                    first_digit = i as i64;
                }
            }

            //Similalrly, search for later words
            if Option::is_some(&last_digits[i]) {
                let target_index = last_digits[i].unwrap();
                if target_index > last_index {
                    last_index = target_index;
                    last_digit = i as i64;
                }
            } 
        }
        p2_sum += 10*first_digit + last_digit
    }

    return (p1_sum.to_string(), p2_sum.to_string());
}