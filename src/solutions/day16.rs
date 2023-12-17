use std::path::Path;

pub fn day16_solution(input_data: String) -> (String, String) {

    let mirror_map = input_data.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let mut pass_counts = vec![vec![0i64; mirror_map[0].len()]; mirror_map.len()];
    let mut path_queue: Vec<Path_Head> = Vec::new(); 
    let mut fork_history: Vec<Path_Head> = Vec::new();

    path_queue.push(Path_Head{x: 0, y: 0, dir: Dirs::Right});

    loop {
        //If no more paths need tracing, then halt
        if path_queue.len() == 0 {
            break;
        }

        let next_path = path_queue.pop().unwrap();
        trace_path(next_path, &mirror_map, &mut pass_counts, &mut path_queue, &mut fork_history);
    }

    let mut p1_result: i64 = 0;

    for i in 0 .. mirror_map.len() {
        for j in 0 .. mirror_map[0].len() {
            if pass_counts[i][j] > 0 {p1_result += 1;}
        }
    }

    return (p1_result.to_string(), "Not Implemented".to_string());
}

fn trace_path(
    path_start: Path_Head, 
    mirror_map: &Vec<Vec<u8>>, 
    pass_counts: &mut Vec<Vec<i64>>, 
    path_queue: &mut Vec<Path_Head>, 
    fork_history: &mut Vec<Path_Head>
) -> () {

    let mut next_point = path_start;

    loop {
        let point = mirror_map[next_point.x][next_point.y];

        match next_point.dir {
            Dirs::Up => {
                match point {
                    b'.' => get_next_head(next_point.x, next_point.y - 1, Dirs::Up, mirror_map)
                }
            }






        }




    }

}

fn get_next_head (pos_x: usize, pos_y: usize, dir: Dirs, mirror_map: &Vec<Vec<u8>>) -> Option<Path_Head> {

    //Catch out of bounds
    if pos_x == 0 && dir == Dirs::Up {return None;}
    if pos_y == 0 && dir == Dirs::Left {return None;}

    if pos_x == mirror_map.len() - 1 && dir == Dirs::Down {return None;}
    if pos_y == mirror_map[0].len() - 1 && dir == Dirs::Right {return None;}

    match dir {
        Dirs::Up => Some(Path_Head{x: pos_x - 1, y: pos_y, dir: dir}),
        Dirs::Down => Some(Path_Head{x: pos_x + 1, y: pos_y, dir: dir}),
        Dirs::Left => Some(Path_Head{x: pos_x, y: pos_y - 1, dir: dir}),
        Dirs::Right => Some(Path_Head{x: pos_x, y: pos_y + 1, dir: dir}),
    }
}

#[derive(PartialEq)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
struct Path_Head {
    x: usize,
    y: usize,
    dir: Dirs,
}