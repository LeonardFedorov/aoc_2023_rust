pub fn day17_solution(input_data: String) -> (String, String) {

    let map: Vec<Vec<u8>> = input_data.lines().map(
        |line| 
            line.bytes().map(
                |c| c - b'0'
            ).collect::<Vec<u8>>()
    ).collect();    

    let goal = Point{row: map.len() - 1, col: map[0].len() - 1};

    //Set up a map to track the lowest score recorded at each point
    let mut min_paths: Vec<Vec<Vec<Path>>> = vec![vec![Vec::new(); map[0].len()]; map.len()];
    let mut dir_paths: Vec<Vec<Option<Dirs>>> = vec![vec![None; map[0].len()]; map.len()];

    //Set up the first point
    let mut path_queue: Vec<Path> = Vec::new();
    path_queue.push(Path{
        pos: Point{row: 0, col: 0}, 
        curr_weight: 0, 
        projected_weight: heuristic(Point {row: 0, col: 0} , goal), 
        straight_count: 0, 
        came_from: Dirs::Null,
    });

    let p1_result = loop {
        let next_point = path_queue.pop().expect("Queue depleted unexpectedly");
        dir_paths[next_point.pos.row][next_point.pos.col] = Some(next_point.came_from);

        //If the end point is reached, return the weight as the result
        if next_point.pos == goal {
            break next_point.curr_weight;
        }

        let next_points = valid_dirs(&next_point, &map, &mut min_paths);

        for i in 0 .. next_points.len() {
            path_queue.push(next_points[i]);
        }

        //Where multiple points exist with the same weight, take the one with the longest straight available
        path_queue.sort_unstable_by_key(|p| - (p.projected_weight * 10 + 3 - p.straight_count));

    };

    for i in 0 .. dir_paths.len() {
        for j in 0 .. dir_paths[0].len() {
            let point = if dir_paths[i][j].is_some() {
                match dir_paths[i][j].unwrap() {
                    Dirs::Down => "A",
                    Dirs::Up => "V",
                    Dirs::Left => ">",
                    Dirs::Right => "<",
                    Dirs::Null => "!",
                }
            } else {
                "-"
            };

            print!(" {point} ");
        }
        print!("\n");
    }

    return (p1_result.to_string(), "Not Implemented".to_string());
}

fn heuristic(curr_point: Point, goal: Point) -> i64 {
    (goal.row - curr_point.row + goal.col - curr_point.col) as i64
}

fn valid_dirs(path: &Path, map: &Vec<Vec<u8>>, min_map: &mut Vec<Vec<Vec<Path>>>) -> Vec<Path> {

    let mut result: Vec<Path> = Vec::with_capacity(4);
    let goal = (map.len() - 1, map[0].len() - 1);
    let max_straight = 3;

    //Go up?
    if path.row > 0 && path.came_from != Dirs::Up {
        let straight_count = if path.came_from == Dirs::Down {path.straight_count + 1} else {0};
        if straight_count <= max_straight {
            let new_row = path.row - 1;
            let new_col = path.col;
            let new_weight = path.curr_weight + map[new_row][new_col] as i64;
            let projected_weight = new_weight + heuristic(new_row, new_col, goal);

            let new_path = Path{
                row: new_row,
                col: new_col,
                curr_weight: new_weight,
                straight_count: straight_count,
                projected_weight: projected_weight,
                came_from: Dirs::Down,
            };

            if !redundant_path(&min_map[new_row][new_col], new_path) {
                min_map[new_row][new_col].push(new_path);
                result.push(new_path);
            }
        }
    }
    //Go down?
    if path.row < map.len() - 1 && path.came_from != Dirs::Down {
        let straight_count = if path.came_from == Dirs::Up {path.straight_count + 1} else {0};
        if straight_count <= max_straight {
            let new_row = path.row + 1;
            let new_col = path.col;
            let new_weight = path.curr_weight + map[new_row][new_col] as i64;
            let projected_weight = new_weight + heuristic(new_row, new_col, goal);

            let new_path = Path{
                row: new_row,
                col: new_col,
                curr_weight: new_weight,
                straight_count: straight_count,
                projected_weight: projected_weight,
                came_from: Dirs::Up,
            };

            if !redundant_path(&min_map[new_row][new_col], new_path) {
                min_map[new_row][new_col].push(new_path);
                result.push(new_path);
            }
        }
    }
    //Go left?
    if path.col > 0 && path.came_from != Dirs::Left {
        let straight_count = if path.came_from == Dirs::Right {path.straight_count + 1} else {0};
        if straight_count <= max_straight {
            let new_row = path.row;
            let new_col = path.col - 1;
            let new_weight = path.curr_weight + map[new_row][new_col] as i64;
            let projected_weight = new_weight + heuristic(new_row, new_col, goal);

            let new_path = Path{
                row: new_row,
                col: new_col,
                curr_weight: new_weight,
                straight_count: straight_count,
                projected_weight: projected_weight,
                came_from: Dirs::Right,
            };

            if !redundant_path(&min_map[new_row][new_col], new_path) {
                min_map[new_row][new_col].push(new_path);
                result.push(new_path);
            }
        }
    }
    //Go right?
    if path.col < map[0].len() - 1 && path.came_from != Dirs::Right {
        let straight_count = if path.came_from == Dirs::Left {path.straight_count + 1} else {0};
        if straight_count <= max_straight {
            let new_row = path.row;
            let new_col = path.col + 1;
            let new_weight = path.curr_weight + map[new_row][new_col] as i64;
            let projected_weight = new_weight + heuristic(new_row, new_col, goal);

            let new_path = Path{
                row: new_row,
                col: new_col,
                curr_weight: new_weight,
                straight_count: straight_count,
                projected_weight: projected_weight,
                came_from: Dirs::Left,
            };

            if !redundant_path(&min_map[new_row][new_col], new_path) {
                min_map[new_row][new_col].push(new_path);
                result.push(new_path);
            }
        }
    }    
    
    return result;
}

fn get_next_path(curr_path: &Path, map: &Vec<Vec<u8>>, dir: Dirs) -> Option<Path> {

    let new_point = curr_path.pos.offset_dir(dir, 1, map.len() - 1, map[0].len() - 1);

    if new_point.is_none() {
        return None;
    }

    let new_point = new_point.unwrap();
    let new_weight = curr_path.curr_weight + new_point.array_access(map) as i64;
    let straight = if dir == curr_path.came_from.rev() {
        curr_path.straight_count + 1
    } else {0};

    if straight > 3 {
        return None;
    }

    return Some(Path {
        pos: new_point,
        curr_weight: new_weight,
        projected_weight: new_weight + heuristic(new_point, Point{row: map.len() - 1, col: map[0].len() - 1}),
        straight_count: straight,
        came_from: dir.rev(),
    });

}

//A path is redundant for testing only if:
//1. The square was visited from the same direction in the past; and
//2. The previous visitation had the same or lower projected weight; and
//3. The previous visitation had the same or higher straight streak running
fn redundant_path(source: &Vec<Path>, new: Path) -> bool {
    let mut result = false;
    if source.len() > 0 {
        for i in 0 .. source.len() {
            let path_compare = source[i];
            if path_compare.came_from == new.came_from &&
                path_compare.projected_weight <= new.projected_weight &&
                path_compare.straight_count <= new.straight_count {
                    result = true;
                    break;
                }
        }
    }
    return result;
}

#[derive(Clone, Copy)]
struct Path {
    pos: Point,
    curr_weight: i64,
    projected_weight: i64,
    straight_count: i64,
    came_from: Dirs,
}

#[derive(PartialEq, Clone, Copy)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
    Null,
}

impl Dirs {
    fn get_cardinals() -> Vec<Dirs> {
        vec![Dirs::Up, Dirs::Left, Dirs::Down, Dirs::Right]
    }

    fn get_other_carindals(&self) -> Vec<Dirs> {
        match self {
            Dirs::Up => vec![Dirs::Left, Dirs::Down, Dirs::Right],
            Dirs::Down => vec![Dirs::Up, Dirs::Left, Dirs::Right],
            Dirs::Left => vec![Dirs::Up, Dirs::Down, Dirs::Right],
            Dirs::Right => vec![Dirs::Up, Dirs::Left, Dirs::Down],
            Dirs::Null => Dirs::get_cardinals(),
        }
    }

    fn rev(&self) -> Dirs {
        match self {
            Dirs::Up => Dirs::Down,
            Dirs::Down => Dirs::Up,
            Dirs::Left => Dirs::Right,
            Dirs::Right => Dirs::Left,
            Dirs::Null => panic!("Cannot reverse the null direction"),
        }        
    }

    fn clockwise(&self) -> Dirs {
        match self {
            Dirs::Up => Dirs::Right,
            Dirs::Down => Dirs::Left,
            Dirs::Left => Dirs::Up,
            Dirs::Right => Dirs::Down,
            Dirs::Null => panic!("Cannot rotate the null direction"),
        }        
    }

    fn anti_clockwise(&self) -> Dirs {
        match self {
            Dirs::Up => Dirs::Left,
            Dirs::Down => Dirs::Right,
            Dirs::Left => Dirs::Down,
            Dirs::Right => Dirs::Up,
            Dirs::Null => panic!("Cannot rotate the null direction"),
        }        
    }

}

#[derive(PartialEq, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn offset_dir(&self, dir: Dirs, count: usize, row_max: usize, col_max: usize) -> Option<Point> {
        
        //Catch underflow
        if self.row < count && dir == Dirs::Up {
            return None;
        }
        if self.col < count && dir == Dirs::Left {
            return None;
        }

        //Generate the new point
        let new_point = match dir {
            Dirs::Up => Point{row: self.row - count, col: self.col},
            Dirs::Down => Point{row: self.row + count, col: self.col},
            Dirs::Left => Point{row: self.row, col: self.col - count},
            Dirs::Right => Point{row: self.row, col: self.col + count},
            Dirs::Null => panic!("Cannot offset by null direction"),
        };

        //Check if it has exceeded the maxes
        if new_point.row > row_max || new_point.col > col_max {
            return None;
        } else {
            return Some(new_point);
        }
    }

    fn array_access<T>(&self, array: &Vec<Vec<T>>) -> T {
        array[self.row][self.col]
    }
}