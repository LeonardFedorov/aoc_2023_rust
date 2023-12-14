pub fn day14_solution(input_data: String) -> (String, String) {

    let mut p1_map = input_data.lines().map(|line| line.bytes().collect()).collect::<Vec<Vec<u8>>>();
    let mut p2_map = p1_map.clone();

    roll_map(&mut p1_map, Dirs::North);
    let p1_result = north_load(&p1_map);

    let mut state_list: Vec<Vec<Vec<u8>>> = Vec::new();

    //Loop through rolling cycles until the map reaches equilibrium
    let cycle_start = loop {
        //Save the previous map
        let p2_map_saved = p2_map.clone();
        state_list.push(p2_map_saved);

        roll_map(&mut p2_map, Dirs::North);
        roll_map(&mut p2_map, Dirs::West);
        roll_map(&mut p2_map, Dirs::South);
        roll_map(&mut p2_map, Dirs::East);

        //Check if the new map is part of the state list
        let cycle_find = state_list.iter().position(|m| compare_maps(m, &p2_map));

        if cycle_find.is_some() {
            break cycle_find.unwrap();
        }
    };

    let cycle_length: usize = state_list.len() - cycle_start;
    let target_index = cycle_start + ((1_000_000_000 - cycle_start) % cycle_length);

    let p2_result = north_load(&state_list[target_index]);

    return (p1_result.to_string(), p2_result.to_string());
}

//Check if two maps are the same
fn compare_maps(map1: &Vec<Vec<u8>>, map2: &Vec<Vec<u8>>) -> bool {

    let mut result = true;
    'outer: for i in 0 .. map1.len() {
        for j in 0 .. map1[0].len() {
            if map1[i][j] != map2[i][j] {
                result = false;
                break 'outer;
            }
        }
    }
    return result;
}

//Mutably roll the map in place
fn roll_map(map: &mut Vec<Vec<u8>>, dir: Dirs) -> () {

    for i in 0 .. map.len() {
        for j in 0 .. map[0].len() {
            //Reverse direction on roll axis is rolling in the positive direction as the farthese things must be done first
            //Ideally I'd have had a conditional iterator here, but didn't bother to figure that
            let target_row = if dir == Dirs::South {map.len() - 1 - i} else {i};
            let target_col = if dir == Dirs::East {map[0].len() - 1 - j} else {j};

            if map[target_row][target_col] == b'O' {
                roll_rock(map, (target_row ,target_col), &dir);
            }
        }
    }

}

fn north_load(map: &Vec<Vec<u8>>) -> usize {
    let mut result = 0;
    for i in 0 .. map.len() {
        for j in 0 .. map[0].len() {
            if map[i][j] == b'O' {result += map.len() - i;}
        }
    }
    return result;
}

fn roll_rock(map: &mut Vec<Vec<u8>>, rock: (usize, usize), dir: &Dirs) -> (usize, usize) {

    let mut position = rock;

    loop {
        //Can the rock roll further?
        //Catch underflow first as we're working in usize
        if *dir == Dirs::North && position.0 == 0 {break;}
        if *dir == Dirs::West && position.1 == 0 {break;}

        let new_position = match dir {
            Dirs::North => (position.0 - 1, position.1),
            Dirs::South => (position.0 + 1, position.1),
            Dirs::West => (position.0, position.1 - 1),
            Dirs::East => (position.0, position.1 + 1),
        };
        //Check for overflow next
        if new_position.0 >= map.len() || new_position.1 >= map[0].len() {
            break;
        }
        //Lastly check for obstruction
        if map[new_position.0][new_position.1] != b'.' {
            break;
        }

        //Now roll the rock
        map[new_position.0][new_position.1] = b'O';
        map[position.0][position.1] = b'.';
        position = new_position;
    }
    
    return position;

}

#[derive(PartialEq)]
enum Dirs {
    North,
    South,
    West,
    East
}