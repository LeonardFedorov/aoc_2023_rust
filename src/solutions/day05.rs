struct Range {
    source_start: i64,
    source_end: i64,
    destination: i64,
}
struct Map {
    source: String,
    dest: String,
    maps: Vec<Range>,
}

pub fn day05_solution(input_data: String) -> (String, String) {

    let mut split_data = input_data.split("\r\n\r\n");

    let seed_list: Vec<i64> = split_data
        .nth(0)
        .expect("Data split totally failed")
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<i64>().expect("Int parse failed"))
        .collect();

    //Parse the maps
    let mut parsed_maps: Vec<Map> = Vec::with_capacity(seed_list.len());

    for map in split_data {

        let mut map = map.lines();

        //Extract the first line to get the location data
        let first_line = map
            .nth(0).expect("Map description invalid")
            .split_ascii_whitespace()
            .nth(0).expect("Map description invalid")
            .split('-')
            .collect::<Vec<&str>>();

        let source = first_line[0].to_string();
        let destination = first_line[2].to_string();

        //Now, get the list of the mappings in this map
        let mut result_map: Vec<Range> = Vec::with_capacity(map.clone().count());

        for range in map {
            let split: Vec<i64> = range.split_ascii_whitespace().map(|num| num.parse::<i64>().expect("Int parse failed")).collect();
            result_map.push(Range {source_start: split[1], source_end: split[1] + split[2], destination: split[0]});
        }

        parsed_maps.push(Map {source: source, dest: destination, maps: result_map});
    }

    //Calculate part 1 answer
    let mut p1_result: Option<i64> = None; 
    for i in 0 .. seed_list.len() {

        let (value, _headroom) = trace_seed(seed_list[i], &parsed_maps);
        //p1_result is null if no values have been found so far, so accept value automatically
        p1_result = option_min(p1_result, value);
    }

    //Calculate part 2 answer
    let mut p2_result: Option<i64> = None; 

    assert!(seed_list.len() % 2 == 0);
    for i in 0 .. seed_list.len()/2 {
        let mut j = seed_list[2*i];
        while j < seed_list[2*i] + seed_list [2*i + 1] {
                        
            let (value, headroom) = trace_seed(j, &parsed_maps);
            //p2_result is null if no values have been found so far, so accept value automatically
            p2_result = option_min(p2_result, value);

            if headroom.is_none() {break;}
            j += headroom.unwrap().max(1);
        }
    }

    return (p1_result.unwrap().to_string(), p2_result.unwrap().to_string());
}

//Trace a given seed
//This code does not assume that the maps are in the right order in the file, even though in my case they are
//First returned int is the destination value, the second return number is the range headroom for part 2
fn trace_seed(seed: i64, parsed_maps: &Vec<Map>) -> (i64, Option<i64>) {
    let mut location = "seed".to_string();
    let end_point = "location".to_string();
    let mut value: i64 = seed;

    let mut headroom: Option<i64> = None;

    //Loop through the maps until we reach the end point
    loop {
        let map = parsed_maps.iter().find(|map| map.source == location).expect("Couldn't find relevant map");
        
        for range in map.maps.iter() {
            if value >= range.source_start && value < range.source_end {

                //Record the headroom
                headroom = option_min(headroom, range.source_end - value);

                //Update value                  
                value = range.destination + value - range.source_start;
                break;
            } else if range.source_start > value {
                headroom = option_min(headroom, range.source_start - value);
            } 
        }
        //Update location and check if we've reached the end
        location = map.dest.clone();
        if location == end_point {break;}
    }

    return (value, headroom);
}

fn option_min(option_val: Option<i64>, new_val: i64) -> Option<i64> {
    return if option_val.is_none() {Some(new_val)}
            else {Some(option_val.unwrap().min(new_val))}  
}