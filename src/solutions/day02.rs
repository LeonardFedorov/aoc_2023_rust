pub fn day02_solution(input_data: String) -> (String, String) {

    let p1_limits = vec![12, 13, 14];
    let rgb = vec!["red", "green", "blue"];

    let mut p1_result: i64 = 0;
    let mut p2_result: i64 = 0;    

    for game in input_data.lines() {

        //Strip off the leading game number, but extract the ID first
        let colon_pos = game.find(":").expect("Invalid game line");
        let game_prefix: Vec<&str> = game[..colon_pos].split(" ").collect();
        let game_id = i64::from_str_radix(game_prefix[1],10).expect("Invalid gameID");

        //Extract the draw text and set up mutable variables to track the result
        let game = &game[colon_pos + 2..];
        let mut game_valid = true;
        let mut cube_mins = vec![0; rgb.len()];

        //Analyse each cube colour statement separately
        for draw in game.split("; ") {
            for cubes in draw.split(", ") {

                let space_pos = cubes.find(" ").expect("invalid cube string");
                let number = i64::from_str_radix(&cubes[0..space_pos], 10).expect("Invalid colour number");
                let colour = rgb.iter().position(|c| **c == cubes[space_pos + 1 ..]).expect("Invalid colour name");

                if number > p1_limits[colour] {game_valid = false;} //Is the game valid for the purposes of day 1?
                cube_mins[colour] = cube_mins[colour].max(number); //Update our cube colour tracking array
            }
        }
        //Update the result tracking variables
        if game_valid {p1_result += game_id};
        let game_power: i64 = cube_mins.iter().fold(1, |acc , n| acc * n);
        p2_result += game_power;
    }

    return (p1_result.to_string(), p2_result.to_string());
}