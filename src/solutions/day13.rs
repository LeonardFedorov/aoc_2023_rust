pub fn day13_solution(input_data: String) -> (String, String) {

    let maps = input_data.split("\r\n\r\n").map(
        |block| block.lines().map(
            |line| line.as_bytes()
        ).collect::<Vec<&[u8]>>()
    );

    let mut p1_result: i64 = 0;
    let mut p2_result = 0;
    
    'map_test: for map in maps {

        //Test columns the i index indicates how many col are left of the line being tested for reflection
        for i in 1 .. map[0].len() {
            //How many cols need to be tested for this col
            let test_width = i.min(map[0].len() - i);
            let mut col_reflection = true;       
            let mut parts_done = 0;
            let mut p2_col_error_count = 0;

            'col_test: for j in 1 ..= test_width {

                for k in 0 .. map.len() {
                    if map[k][i - j] != map[k][i + j - 1] {
                        col_reflection = false;
                        p2_col_error_count += 1;
                        // only stop checking this config after it can't satisfy either part
                        if p2_col_error_count == 2 {break 'col_test;}
                    }
                }
            }

            if col_reflection {
                p1_result += i as i64;
                parts_done += 1;
            }

            if p2_col_error_count == 1 {
                p2_result += i as i64;
                parts_done += 1;
            }

            //Once we have our answer, go to the next map
            if parts_done == 2 {
                continue 'map_test;
            }

        }
    
        //the i index indicates how many rows are above the line being tested for reflection
        for i in 1 .. map.len() {
            //How many rows need to be tested for this row
            let test_width = i.min(map.len() - i);
            let mut row_reflection = true;       
            let mut parts_done = 0;
            let mut p2_row_error_count = 0;

            'row_test: for j in 1 ..= test_width {
                for k in 0 .. map[0].len() {
                    if map[i - j][k] != map[i + j - 1][k] {
                        row_reflection = false;
                        p2_row_error_count += 1;
                        // only stop checking this config after it can't satisfy either part
                        if p2_row_error_count == 2 {break 'row_test;}
                    }
                }
            }

            if row_reflection {
                p1_result += 100 * i as i64;
                parts_done += 1;
            }

            if p2_row_error_count == 1 {
                p2_result += 100 * i as i64;
                parts_done += 1;
            }

            //Once we have our answer, go to the next map
            if parts_done == 2 {
                continue 'map_test;
            }
        }

    }

    return (p1_result.to_string(), p2_result.to_string());
}