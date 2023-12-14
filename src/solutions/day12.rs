pub fn day12_solution(input_data: String) -> (String, String) {

    let input_data_processed: Vec<(Vec<u8>, Vec<usize>)> = input_data.lines().map(
        |line| line.split(' ').collect::<Vec<&str>>()
    ).map(
        |line_parts| 
            (
                condense_map(line_parts[0]), 
                line_parts[1].split(',').map(|x| x.parse::<usize>().expect("Integer parse failed")).collect::<Vec<usize>>()
            )
    ).collect();

    //Part 1
    let mut p1_result: usize = 0;

    for i in 0 .. input_data_processed.len() {

        let spring_map = &input_data_processed[i].0;
        let group_lens = &input_data_processed[i].1;

        p1_result += solve_combinations(spring_map, group_lens);

    }


    return (p1_result.to_string(), "Not Implemented".to_string());

}

fn condense_map(spring_map: &str) -> Vec<u8> {

    //Condense the map by trimming operational springs and reducing blocks of operational springs to 1 spring wide
    let spring_map = spring_map.as_bytes();
    let mut spring_map_condensed: Vec<u8> = Vec::with_capacity(spring_map.len());
    
    for j in 0 .. spring_map.len() {
        if spring_map_condensed.len() == 0 && spring_map[j] == b'.' {
            //Do nothing - trim the start
        } else if spring_map[j] == b'.' && spring_map[j-1] == b'.' {
            //Do nothing
        } else {
            spring_map_condensed.push(spring_map[j]);
        }
    }

    //Trim .s at the end
    while *spring_map_condensed.last().expect("Condensed map is empty") == b'.' {
        spring_map_condensed.pop();
    }

    return spring_map_condensed;

}

//This function assumes the positioning is solvable, and thus doesn't check for impossibility edge cases
fn solve_combinations(spring_map: &Vec<u8>, group_lens: &Vec<usize>) -> usize {

    //First, find the upper bound for the position of each item by trying to densely pack from the right
    let mut upper_bounds = vec![0; group_lens.len()];

    let mut start_point = spring_map.len() - group_lens.last().expect("Invalid group_lens vector");

    for i in (0 .. group_lens.len()).rev() {

        if start_point == 0 {
            upper_bounds[0] = 0;
        } else {
            for j in (0 ..= start_point).rev() {
                
                if spring_map[j-1] == b'#' {
                    //Do nothing, can't place here because there must be a broken spring adjacent
                }
                else if spring_map[j..j+group_lens[i]].contains(&b'.') {
                    //Do nothing, this position is invalid
                } else {
                    //The section can fit here, so save it down and end the j loop
                    upper_bounds[i] = j;
                    if i > 0 {start_point = j - 1 - group_lens[i - 1];}
                    break;
                }
            }
        }
    }



    
    return upper_bounds.len();
}