struct Point {
    row: usize,
    col: usize,
}
struct NumberResult {
    value: i64,
    is_part: bool,
    length: usize,
}

pub fn day03_solution(input_data: String) -> (String, String) {

    let input_grid: Vec<Vec<char>> = input_data.lines().map(|line| line.chars().collect()).collect();
    let row_count = input_grid.len() as usize;
    let col_count = input_grid[0].len() as usize;

    let mut gear_map: Vec<Vec<Option<i64>>> = vec![vec![None; col_count]; row_count];
    let mut gear_list: Vec<Point> = Vec::new();

    let mut p1_result: i64 = 0;
    let mut p2_result: i64 = 0;

    //Evaluate part 1 and build the map for processing part 2
    let mut pointer = Point {row: 0, col: 0};
    loop {

        //Check if the point is a digit, if so process as a number for part 1
        if input_grid[pointer.row][pointer.col].is_digit(10) {

            let number = process_number(&pointer, &input_grid);
            if number.is_part {p1_result += number.value;}
            
            //Save the number's values into the gear map for use in part 2
            for i in pointer.col .. pointer.col + number.length {
                gear_map[pointer.row][i] = Some(number.value);
            }

            //Skip pointer ahead by the length of the number
            pointer.col += number.length;

        //If the point is a gear, store its position for later
        } else if input_grid[pointer.row][pointer.col] == '*'{
            gear_list.push(Point {row: pointer.row, col:pointer.col});
            pointer.col += 1;
        } else {
            pointer.col += 1;
        }

        //If we have reached the end of a row, go to the next one
        if pointer.col >= col_count {
            pointer.col = 0;
            pointer.row += 1;
        }

        //If we have finished the final row, end the loop
        if pointer.row >= row_count {
            break;
        }
    }

    for gear in gear_list.iter() {

        let mut number_count: i64 = 0;
        let mut number_product: i64 = 1;

        //Flatten the neighbours into a single list for ease of use, and handle index bounds as well
        let mut flat_neighbours: Vec<Option<i64>> = vec![None; 8];

        //Points above the gear
        if gear.row > 0 {
            if gear.col > 0 {
                flat_neighbours[0] = gear_map[gear.row - 1][gear.col - 1];
            }
            flat_neighbours[1] = gear_map[gear.row - 1][gear.col];
            if gear.col < col_count - 1 {
                flat_neighbours[2] = gear_map[gear.row - 1][gear.col + 1];
            }
        }
        //Points either side of the gear
        if gear.col > 0 {
            flat_neighbours[3] = gear_map[gear.row][gear.col - 1];
        } 
        if gear.col < col_count - 1 {
            flat_neighbours[4] = gear_map[gear.row][gear.col + 1];
        }
        //Points below the gear
        if gear.row < row_count - 1 {
            if gear.col > 0 {
                flat_neighbours[5] = gear_map[gear.row + 1][gear.col - 1];
            }
            flat_neighbours[6] = gear_map[gear.row + 1][gear.col];
            if gear.col < col_count - 1 {
                flat_neighbours[7] = gear_map[gear.row + 1][gear.col + 1];
            }
        }

        //Now check which adjacent points contain numbers
        //Key realisation: if the top middle or bottom middle contain a number, then top/bottom left/right dont need to be checked
        if !update_gear(flat_neighbours[1], &mut number_count, &mut number_product) {
            for i in [0,2] {
                update_gear(flat_neighbours[i], &mut number_count, &mut number_product);             
            }
        }

        for i in [3,4] {
            update_gear(flat_neighbours[i], &mut number_count, &mut number_product);
        }

        if !update_gear(flat_neighbours[6], &mut number_count, &mut number_product) {
            for i in [5,7] {
                update_gear(flat_neighbours[i], &mut number_count, &mut number_product);             
            }
        }
        
        if number_count == 2 {p2_result += number_product;}

    }

    return (p1_result.to_string(), p2_result.to_string());
}

fn process_number(start: &Point, input_grid: &Vec<Vec<char>>) -> NumberResult {

    //First extract the number value
    let mut value: i64 = 0;
    let mut length: usize = 0;
    let row = &input_grid[start.row];
    let row_length = row.len();

    //Iterate forward in the string until we stop finding digits
    while (start.col + length < row_length) && row[start.col + length].is_digit(10) {
        value *= 10;
        value += (row[start.col + length] as i64) - 0x30;
        length += 1;
    }

    //Now, determine if there is an adjacent symbol
    //Since we don't care about multiples, skip all further checks if it is found to be a part
    let mut is_part = false;

    //Check to the left and the right
    if !is_part && start.col > 0 {
        if is_symbol(&input_grid[start.row][start.col - 1]) {
            is_part = true;
        }
    }

    if !is_part && start.col + length < (row_length - 1) {
        if is_symbol(&input_grid[start.row][start.col + length]) {
            is_part = true;
        }        
    }

    //Check neighbouring rows
    let start_col = if start.col == 0 {start.col} else {start.col - 1};
    let end_col = if start.col + length == row_length {start.col + length - 1} else {start.col + length};

    //Above...
    if !is_part && start.row > 0 {
        for i in start_col ..= end_col {
            if is_symbol(&input_grid[start.row - 1][i]) {
                is_part = true;
                break;
            }
        }
    }    

    //...and below
    if !is_part && start.row < input_grid.len() - 1 {
        for i in start_col ..= end_col {
            if is_symbol(&input_grid[start.row + 1][i]) {
                is_part = true;
                break;
            }
        }
    }    

    return NumberResult {value: value, is_part: is_part, length: length};

}

fn is_symbol(c: &char) -> bool {
    if c.is_digit(10) || *c == '.' {
        return false;
    } else {
        return true;
    }
}

//This function mutabily alters the passed variables and exists to avoid repeated code
fn update_gear(neighbour: Option<i64>, number_count: &mut i64, number_product: &mut i64) -> bool {
    if neighbour.is_some() {
        *number_count += 1;
        *number_product *= neighbour.unwrap();
    }    
    return neighbour.is_some();
}