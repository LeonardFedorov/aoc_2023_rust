pub fn day10_solution(input_data: String) -> (String, String) {

    let pipe_config: Vec<Vec<Dirs>> = get_pipe_config();

    let mut pipe_map = input_data.lines().map(
        |line| 
            line.bytes().map(
                |c|
                    match c {
                        b'|' => Pipes::Vertical,
                        b'-' => Pipes::Horizontal,
                        b'L' => Pipes::NE,
                        b'J' => Pipes::NW,
                        b'7' => Pipes::SW,
                        b'F' => Pipes::SE,
                        b'.' => Pipes::NP,
                        b'S' => Pipes::AN,
                        _ => panic!("Invalid pipe character")
                    }
                ).collect::<Vec<Pipes>>()
    ).collect::<Vec<Vec<Pipes>>>();

    //Find the start
    let mut animal_start = (0,0);

    for i in 0 .. pipe_map.len() {
        for j in 0 .. pipe_map[0].len() {
            if pipe_map[i][j] == Pipes::AN {
                animal_start = (i,j);
                break;
            }
        }
    }

    //Get a list of the adjacent pipes that connect to the start point
    let loop_candidates = get_loop_starts(animal_start, &pipe_map, &pipe_config);

    let mut loop_nodes: Vec<(usize, usize)> = Vec::new();
    let mut start_dirs: Vec<Dirs> = Vec::new();
    
    //Try each candidate in turn to see if it forms a full loop
    for i in 0 .. loop_candidates.len() {

        //Set up initial trace variables
        let mut current_pos = animal_start;
        loop_nodes = Vec::new();
        let mut next_dir = loop_candidates[i];
        let mut cycle_found = true;
        start_dirs = Vec::new();
        start_dirs.push(next_dir);

        loop_nodes.push(current_pos);

        loop {
            //Test if the next point is valid in the grid
            let peek_pos = get_next_node(current_pos, next_dir, &pipe_map);
            if peek_pos.is_none() {
                cycle_found = false;
                break;
            }
            current_pos = peek_pos.unwrap();

            if current_pos == animal_start {
                start_dirs.push(next_dir.invert());
                break;
            } //Loop is now completed

            //Test if the next pipe actually connects back
            let next_pipe = pipe_map[current_pos.0][current_pos.1];
            if does_pipe_connect(next_pipe, next_dir.invert(), &pipe_config) {
                //Get the next direction to go in
                next_dir = get_next_connection(next_dir.invert(), next_pipe, &pipe_config);
                loop_nodes.push(current_pos);        
            } else {
                //Dead end
                cycle_found = false;
                break;
            }
        }
        //If a cycle was found, then stop looking and return
        if cycle_found {break;}
    }

    //Having finished, update the start position with the type of pipe it is now known to be
    let pipe_number = pipe_config.iter().position(
        |pipe| 
            pipe.contains(&start_dirs[0]) && pipe.contains(&start_dirs[1])
        ).expect("Couldn't find valid pipe");

    pipe_map[animal_start.0][animal_start.1] = Pipes::from_int(pipe_number);

    //Part 2
    let mut p2_result: usize = 0;

    //Find bounding box of the path
    let bounding = bound_path(&loop_nodes, &pipe_map);

    //Now search the area, going by row
    for i in bounding.min_row ..= bounding.max_row {

        let mut inside = false;
        let mut on_horizontal = false;
        let mut horizontal_entry: Dirs = Dirs::Up;

        for j in bounding.min_col ..= bounding.max_col {
            
            //Check if current point is on the path
            let path_point = loop_nodes.contains(&(i,j));

            //If we're inside and not on a loop node, increment the counter
            if !path_point {
                if inside {p2_result += 1};
            } else if pipe_map[i][j] == Pipes::Horizontal {
                //Do nothing
            } else if pipe_map[i][j] == Pipes::Vertical {
                inside = !inside;
            } else {
                //Now dealing only with corner pieces
                if !on_horizontal {
                    on_horizontal = true;
                    horizontal_entry = get_vertical_dir(pipe_map[i][j])
                } else {
                    on_horizontal = false;

                    if horizontal_entry == get_vertical_dir(pipe_map[i][j]) {
                        //Do nothing, no cross over so no change on place
                    } else {
                        inside = !inside;
                    }
                }
            }
        }
    }

    return ((loop_nodes.len()/2).to_string(), p2_result.to_string());
}

#[derive(PartialEq, Copy, Clone)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

impl Dirs {
    fn invert(self) -> Dirs {
        match self {
            Dirs::Up => Dirs::Down,
            Dirs::Down => Dirs::Up,
            Dirs::Left => Dirs::Right,
            Dirs::Right => Dirs::Left,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Pipes {
    Vertical = 0,
    Horizontal = 1,
    NE = 2,
    NW = 3,
    SW = 4,
    SE = 5,
    NP = 6,
    AN = 7,
}

impl Pipes {
    fn from_int(i: usize) -> Pipes {
        match i {
            0 => Pipes::Vertical,
            1 => Pipes::Horizontal,
            2 => Pipes::NE,
            3 => Pipes::NW,
            4 => Pipes::SW,
            5 => Pipes::SE,
            6 => Pipes::NP,
            7 => Pipes::AN,
            _ => panic!("Invalid pipe number")
        }
    }
}

fn get_pipe_config() -> Vec<Vec<Dirs>> {
    vec![
        vec![Dirs::Up, Dirs::Down],
        vec![Dirs::Left, Dirs::Right],
        vec![Dirs::Up, Dirs::Right],
        vec![Dirs::Up, Dirs::Left],
        vec![Dirs::Down, Dirs::Left],
        vec![Dirs::Down, Dirs::Right],
        vec![],
        vec![],  
    ]
}

fn get_loop_starts(start_pos: (usize, usize), pipe_map: &Vec<Vec<Pipes>>, pipe_config: &Vec<Vec<Dirs>>) -> Vec<Dirs> {

    let mut result: Vec<Dirs> = Vec::with_capacity(4);
    if start_pos.0 > 0 {
        if does_pipe_connect(pipe_map[start_pos.0 - 1][start_pos.1], Dirs::Down, pipe_config) {
            result.push(Dirs::Up);
        }
    }
    if start_pos.0 < pipe_map[0].len() - 1 {
        if does_pipe_connect(pipe_map[start_pos.0 + 1][start_pos.1], Dirs::Up, pipe_config) {
            result.push(Dirs::Down);
        }
    }
    if start_pos.1 > 0 {
        if does_pipe_connect(pipe_map[start_pos.0][start_pos.1 - 1], Dirs::Right, pipe_config) {
            result.push(Dirs::Left);
        }
    }
    if start_pos.1 < pipe_map.len() - 1 {
        if does_pipe_connect(pipe_map[start_pos.0][start_pos.1 + 1], Dirs::Left, pipe_config) {
            result.push(Dirs::Right);
        }
    }
    return result;
}

fn does_pipe_connect(pipe: Pipes, direction: Dirs, pipe_config: &Vec<Vec<Dirs>>) -> bool {

    let this_pipe: &Vec<Dirs> = &pipe_config[pipe as usize];
    return this_pipe.contains(&direction);

}

fn get_next_node(position: (usize, usize), direction: Dirs, pipe_map: &Vec<Vec<Pipes>>) -> Option<(usize, usize)> {
    match direction {
        Dirs::Up => if position.0 == 0 {None} else {Some((position.0 - 1, position.1))},
        Dirs::Down => if position.0 == pipe_map.len() {None} else {Some((position.0 + 1, position.1))},
        Dirs::Left => if position.1 == 0 {None} else {Some((position.0, position.1 - 1))},
        Dirs::Right => if position.1 == pipe_map[0].len() {None} else {Some((position.0, position.1 + 1))},
    }
}

fn get_next_connection(entry: Dirs, pipe: Pipes, pipe_config: &Vec<Vec<Dirs>>) -> Dirs {

    let pipe_deets = pipe_config[pipe as usize].iter();
    let output: Vec<&Dirs> = pipe_deets.filter(|x| **x != entry).collect();

    assert!(output.len() == 1);
    return *output[0];
}

fn get_vertical_dir(pipe: Pipes) -> Dirs {
    match pipe {
        Pipes::NE => Dirs::Up,
        Pipes::NW => Dirs::Up,
        Pipes::SE => Dirs::Down,
        Pipes::SW => Dirs::Down,
        _ => panic!("Tried to get vertical of non-corner pipe"),
    }
}

struct BoundingBox {
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
}

fn bound_path(path: &Vec<(usize, usize)>, pipe_map: &Vec<Vec<Pipes>>) -> BoundingBox {

    let mut result = BoundingBox{min_row: pipe_map.len(), max_row: 0, min_col: pipe_map[0].len(), max_col: 0};

    for i in 0 .. path.len() {
        let (curr_row, curr_col) = path[i];

        result.min_row = result.min_row.min(curr_row);
        result.max_row = result.max_row.max(curr_row);
        result.min_col = result.min_col.min(curr_col);
        result.max_col = result.max_col.max(curr_col);
    }

    return result;
}