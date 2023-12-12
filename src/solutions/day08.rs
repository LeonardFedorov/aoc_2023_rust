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

    //Part 2 - get the start locations that start in A and process their cycle deets
    //This has to be done from original data array because there's no way to discern an empty hash map entry from the true AAA
    let p2_locations = path_data.lines().filter(
        |map_entry|
            map_entry.bytes().collect::<Vec<u8>>()[2] == b'A'
    ).map(
        |a_map_entry|
            convert_address(&a_map_entry[0..=2])
    ).map(
        |a_start: usize| 
            parse_cycle(a_start, &path_map, &directions)
    ).collect::<Vec<CycleDeets>>();

    let mut step_counts: Vec<usize> = vec![0; p2_locations.len()];

    //Implicitly assuming now that there is only one z point in each loop. 
    //This was true from step through inspection though makes the solution not fully general
    for i in 0 .. p2_locations.len() {
        step_counts[i] = p2_locations[i].z_points[0];
    }

    let mut new_max = step_counts[1];
    loop {
        //Loop until alignment found across all paths
        while step_counts[0] < new_max {
            step_counts[0] += p2_locations[0].cycle_length;
        }

        let mut loop_over = true;

        for i in 1 .. step_counts.len() {
            while step_counts[i - 1] > step_counts[i] {
                step_counts[i] += p2_locations[i].cycle_length;
            }

            if step_counts[i - 1] != step_counts[i] {
                new_max = step_counts[i];
                loop_over = false;
                break; //start again
            }
        }
        if loop_over {break;}
    }


    return (p1_step_count.to_string(), step_counts[0].to_string());
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

fn parse_cycle(start_point: usize, path_map: &Vec<Vec<usize>>, directions: &Vec<usize>) -> CycleDeets {

    let mut path_trace: Vec<(usize,usize)> = Vec::with_capacity(directions.len());
    let mut z_points: Vec<usize> = Vec::new();

    let mut location: usize = start_point;

    let mut _cycle_search: Option<usize> = None; //Flagged as unused to prevent compiler warning since it doesn't realise
                                                 //the read inside the loop is inevitable
    path_trace.push((location, 0));
    let mut direction: usize = 0;

    loop {
        location = path_map[location][directions[direction]];

        //Update the direction now as we want to consider this from the PoV of "direction we would go from this point"
        if direction == directions.len() - 1 {
            direction = 0;
        } else {
            direction += 1;
        }

        _cycle_search = path_trace.iter().position(|(loc, dir)| *loc == location && *dir == direction);     

        if _cycle_search.is_some() {
            return CycleDeets {
                cycle_length: (path_trace.len() - _cycle_search.unwrap()),
                z_points: z_points,
            }
        }

        //Store point in the path list
        path_trace.push((location, direction));

        //If this place ends in a Z (and it wasn't part of the cycle), then store it in the Z points
        //Subtract one since the number of points in the path is one greater than the number of steps taken
        //as it includes the start
        if location % 26 == 25 {
            z_points.push(path_trace.len() - 1);
        }

    }

}

struct CycleDeets {
    cycle_length: usize,
    z_points: Vec<usize>,
}