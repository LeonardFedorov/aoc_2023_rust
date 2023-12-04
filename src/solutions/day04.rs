pub fn day04_solution(input_data: String) -> (String, String) {

    //Store as a vector rather than an iterator as it will be easier to loop through explciitly with index
    let mut lines: Vec<_> = input_data.lines().map(|line| line.split([':', '|'])).collect();
    let mut p1_result: i64 = 0;

    let card_count = lines.len();
    let mut scoring_counts: Vec<i64> = vec![1; card_count]; //Initially have 1 of every card

    for i in 0..card_count {

        //Card number is not needed, but need to pull it out of the iterator anyway
        let _card_number = lines[i].nth(0).expect("No card number!");

        //Store winning numbers as a vector so it can be repeatedly searched easily
        let winning_numbers = 
            lines[i]
            .nth(0).expect("No winning numbers!")
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().expect("Invalid number"))
            .collect::<Vec<i64>>();
            
        //Store game_numbers as an iterator so it can be looped through
        let game_numbers = 
            lines[i]
            .nth(0).expect("No game numbers!")
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().expect("Invalid number"));

        let mut matches = 0;
        for number in game_numbers {
            if winning_numbers.contains(&number) {matches += 1;}
        }
        
        //Compute part 1 score
        let result = 
            if matches == 0 {0}
            else {1 << (matches - 1)};
        p1_result += result;

        //Update scoring array for part 2
        for j in (i+1)..=i+matches {
            scoring_counts[j] += scoring_counts[i];
        }
    }

    let p2_result: i64 = scoring_counts.iter().sum();

    return (p1_result.to_string(), p2_result.to_string());
}