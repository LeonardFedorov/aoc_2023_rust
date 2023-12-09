pub fn day08_solution(input_data: String) -> (String, String) {

    let mut data = input_data.split("\r\n\r\n");

    let directions: Vec<usize> = data.next().expect("Unexpected input data").chars().map(
        |direction| 
            match direction {
                'L' => 0usize,
                'R' => 1usize,
                _ => panic!("Invalid direction character"),
            }
    ).collect();

    let path_data = data.next().expect("Unexpected input data");

    let path_map = build_path_map(path_data);    

    //Part 1
    let mut location = convert_address("AAA");
    let destination = convert_address("ZZZ");
    let mut p1_step_count: i64 = 0;
    let mut direction: usize = 0;

    loop {
        //Update looping variables
        p1_step_count += 1;

        location = path_map[location][directions[direction]];
        if location == destination {break;}

        //Set up next direction
        if direction == directions.len() - 1 {
            direction = 0;
        } else {
            direction += 1;
        }
    }

    //Part 2 - get the start locations that start in A
    //This has to be done from original data array because there's no way to discern an empty hash map entry from the true AAA
    let mut p2_locations = path_data.lines().filter(
        |map_entry|
            map_entry.bytes().collect::<Vec<u8>>()[2] == b'A'
    ).map(
        |a_map_entry|
            convert_address(&a_map_entry[0..=2])
    ).collect::<Vec<usize>>();

    let mut p2_step_count = 0;
    direction = 0;

    loop {
        //Update looping variables
        p2_step_count += 1;

        for i in 0 .. p2_locations.len() {
            p2_locations[i] = path_map[p2_locations[i]][directions[direction]];
        }

        if is_p2_over(&p2_locations) {break;}

        //Set up next direction
        if direction == directions.len() - 1 {
            direction = 0;
        } else {
            direction += 1;
        }
    }

    return (p1_step_count.to_string(), p2_step_count.to_string());
}

//Build the path map using a hash map where the hash of a number is its integer representaiton in base 26
//This is VERY much assuming that the input map is self consistent
fn build_path_map(path_data: &str) -> Vec<Vec<usize>> {

    let mut path_map: Vec<Vec<usize>> = vec![vec![0; 2]; 26usize.pow(3)]; 

    for line in path_data.lines() {

        let this_address = convert_address(&line[0..=2]);
        let left_address = convert_address(&line[7..=9]);
        let right_address = convert_address(&line[12..=14]);

        path_map[this_address][0] = left_address;
        path_map[this_address][1] = right_address;
    }

    return path_map;
}

fn convert_address(address: &str) -> usize {

    let as_array = address.bytes().collect::<Vec<u8>>();
    return
          (as_array[0] - b'A') as usize * 26 * 26
        + (as_array[1] - b'A') as usize * 26
        + (as_array[2] - b'A') as usize
    ;
}

//Part 2 is over when every address ends in Z - i.e. when their remainder mod 26 is 25
fn is_p2_over(location_list: &Vec<usize>) -> bool {

    let mut result = true;
    for i in 0.. location_list.len() {
        if location_list[i] % 26 != 25 {
            result = false;
            break;
        }
    }
    return result;
}