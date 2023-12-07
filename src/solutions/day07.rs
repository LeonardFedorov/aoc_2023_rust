use core::panic;
use std::cmp::Ordering;

pub fn day07_solution(input_data: String) -> (String, String) {

    let mut day_results: Vec<i64> = vec![0,0];

    for i in 0 ..= 1 {

        let mut hand_list: Vec<Hand> = input_data.lines().map(
            |line| parse_hand((i + 1) as i64,line)            
        ).collect();

        //Sort the hands in strength order
        hand_list.sort_unstable();

        for j in 0 .. hand_list.len() {
            day_results[i] += hand_list[j].bid * (j as i64 + 1);
        }

    }

    return (day_results[0].to_string(), day_results[1].to_string());
}

fn parse_hand(part: i64, hand_text: &str) -> Hand {
    match part {
        1 => return parse_hand_p1(hand_text),
        2 => return parse_hand_p2(hand_text),
        _ => panic!("Unexpected part number"),
    }
}

fn parse_hand_p1(hand_text: &str) -> Hand {
    
    //Extract the basic data
    let (cards, bid) = extract_hand_data(hand_text);

    //First, convert the cards to a more usable format
    let cards: Vec<u8> = cards.iter().map(
        |card: &u8|
            match card {
                c @ b'2' ..= b'9' => c - b'2',
                b'T' => 8,
                b'J' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => panic!("Illegal card founds") 
            }
    ).collect();

    //Count how many of each card are present
    let mut card_counts = vec![0; 13]; //Where 13 is the number of card types
    for j in 0 .. cards.len() {
        card_counts[cards[j] as usize] += 1;
    }

    //Now count how many pairs, threes, fours and fives appear
    let mut tuple_counts = vec![0; 4];
    for i in 0 .. card_counts.len() {
        match card_counts[i] {
            2 => tuple_counts[0] += 1,
            3 => tuple_counts[1] += 1,
            4 => tuple_counts[2] += 1,
            5 => tuple_counts[3] += 1,
            _ => ()
        }
    }

    let hand_category = evaluate_category(tuple_counts);

    return Hand {cards: cards, bid: bid, category: hand_category};
}

fn parse_hand_p2(hand_text: &str) -> Hand {
    
    //Extract the basic data
    let (cards, bid) = extract_hand_data(hand_text);

    //First, convert the cards to a more usable format
    let cards: Vec<u8> = cards.iter().map(
        |card: &u8|
            match card {
                c @ b'2' ..= b'9' => c - b'2' + 1, // reserve 0 for the
                b'T' => 9,
                b'J' => 0, //joker map override
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => panic!("Illegal card founds") 
            }
    ).collect();

    //Count how many of each card are present
    let mut card_counts = vec![0; 13]; //Where 13 is the number of card types
    for j in 0 .. cards.len() {
        card_counts[cards[j] as usize] += 1;
    }

    //Now count how many pairs, threes, fours and fives appear, but don't count jokers yet
    let mut tuple_counts: Vec<i64> = vec![0; 4];
    for i in 1 .. card_counts.len() {
        match card_counts[i] {
            2 => tuple_counts[0] += 1,
            3 => tuple_counts[1] += 1,
            4 => tuple_counts[2] += 1,
            5 => tuple_counts[3] += 1,
            _ => ()
        }
    }

    if card_counts[0] == 0 {
        //If no jokers, no adjustments needed
    } else if card_counts[0] == 5 {
        //If there are 5 jokers, handle this directly. Otherwise, proceed to find a card type map jokers onto
        tuple_counts[3] = 1;
    } else {
        //Get max tuple size present
        let mut max_tuple = 1;
        for i in (0 ..= 3).rev() {
            if tuple_counts[i] > 0 {max_tuple = i + 2; break;}
        }

        //Update the highest sized tuple to add on the jokers
        tuple_counts[max_tuple + card_counts[0] - 2] += 1;
        if max_tuple > 1 {
            tuple_counts[max_tuple - 2] -= 1;
        }

    }

    let hand_category = evaluate_category(tuple_counts);

    return Hand {cards: cards, bid: bid, category: hand_category};
}

fn extract_hand_data(hand_text: &str) -> (Vec<u8>, i64) {

    let mut line_parts = hand_text.split(' ');
    let cards: Vec<u8> = line_parts.nth(0).expect("Line parse failed").bytes().collect();
    let bid = line_parts.nth(0).expect("Line parse failed").parse::<i64>().expect("Bid parse failed");

    return (cards, bid); 
}

fn evaluate_category(tuple_counts: Vec<i64>) -> HandType {
    
    if tuple_counts[3] == 1 {return HandType::Five;}
    else if tuple_counts[2] == 1 {return HandType::Four;}
    else if tuple_counts[1] == 1 {
        if tuple_counts[0] == 1 {
            return HandType::FullHouse;
        } else {
            return HandType::Three;
        }
    } else if tuple_counts[0] == 2 {
        return HandType::TwoPair;
    } else if tuple_counts[0] == 1 {
        return HandType::Pair;
    } else {
        return HandType::HighCard;
    };
}

//Custom types for this day
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    Three = 3,
    FullHouse = 4,
    Four = 5,
    Five = 6,
}

#[derive(Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: i64,
    category: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {

        //Catch the easy to identify cases first
        if self == other {return Ordering::Equal;}
        if self.category != other.category {
            return self.category.cmp(&other.category);
        }

        //Check character by character until a difference is found
        for i in 0 .. self.cards.len() {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }

        //This return statement should be unreachable
        return Ordering::Equal;
    }
}

//Partial order defined by reference to total order
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    } 
}

//Two hands are the same iff they consist of the same cards in the same order
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards == other.cards;
    }
}