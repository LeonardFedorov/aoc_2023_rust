pub fn day09_solution(input_data: String) -> (String, String) {

    let mut p1_result = 0;
    let mut p2_result = 0;

    for line in input_data.lines() {

        let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
        let seq_len = split_line.len();

        //The implicit assumption here is that these are all polynomial sequences so will reach 0 common differences within their length.
        let mut differences: Vec<Vec<i64>> = vec![vec![0; seq_len]; seq_len];

        //Read the integers into the top row
        for i in 0 .. seq_len {
            differences[0][i] = split_line[i].parse::<i64>().expect("Invalid integer detected");
        }

        let mut all_zeroes = false;
        let mut current_row: usize = 0;

        //Build table of current differences. End when a row of all zeroes is written
        while !all_zeroes {
            current_row += 1;
            all_zeroes = true;

            //Write out differences to row above
            for i in 0 .. seq_len - current_row {
                differences[current_row][i] = differences[current_row - 1][i + 1] - differences[current_row - 1][i];
                if all_zeroes && differences[current_row][i] != 0 {all_zeroes = false;}
            }
        }

        //Now compute the extrapolations
        let mut p1_extrapolation = 0;
        let mut p2_extrapolation = 0;
    
        for i in (0 .. current_row).rev() {
            //Part 1
            p1_extrapolation = differences[i][seq_len - 1 - i] + p1_extrapolation;

            //Part 2
            p2_extrapolation = differences[i][0] - p2_extrapolation; 
        }

        p1_result += p1_extrapolation;
        p2_result += p2_extrapolation;
        
    }

    return (p1_result.to_string(), p2_result.to_string());
}