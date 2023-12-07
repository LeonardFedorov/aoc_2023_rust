pub fn day06_solution(input_data: String) -> (String, String) {

    let mut line_data = input_data.lines();
    let times: Vec<&str> = line_data.nth(0).expect("Unexpected input file format").split_ascii_whitespace().skip(1).collect();
    let distances: Vec<&str> = line_data.nth(0).expect("Unexpected input file format").split_ascii_whitespace().skip(1).collect();

    let mut p2_time = String::new();
    let mut p2_distance = String::new();

    assert!(times.len() == distances.len());

    let mut p1_result = 1;

    for i in 0 .. times.len() {
        p2_time.push_str(times[i]);
        p2_distance.push_str(distances[i]);

        let time = times[i].parse::<f64>().expect("Invalid time number");
        let distance = distances[i].parse::<f64>().expect("Invalid distance number");
        
        let result = get_solution_range(time, distance);

        p1_result *= result;
    }

    let p2_result = get_solution_range(
        p2_time.parse::<f64>().expect("Invalid p2 time"), 
        p2_distance.parse::<f64>().expect("Invalid p2 time")
    );

    return (p1_result.to_string(), p2_result.to_string());
}

//The algebriac methodology is the same as used in the manual solution - see the Day 6 PDF in the repo for an explanation
fn get_solution_range(time_alloted: f64, target_distance: f64) -> i64 {

    let discriminant = (time_alloted.powi(2) - (4.0 * target_distance)).sqrt();
        
    let lower = 0.5 * (time_alloted - discriminant);
    let upper = 0.5 * (time_alloted + discriminant);

    let int_lower = lower.ceil() as i64;
    let int_upper = upper.floor() as i64;

    return int_upper - int_lower + 1; 
}