use core::panic;
use std::cmp::Ordering;

pub fn day07_solution(input_data: String) -> (String, String) {

    let mut hand_list: Vec<Hand> = input_data.lines().map(
        |line| parse_hand(line)            
    ).collect();

    //Sort the hands in strength order
    hand_list.sort_unstable();

    let mut p1_result: i64 = 0;

    for i in 0.. hand_list.len() {
        p1_result += hand_list[i].bid * (i as i64 + 1);
    }

    return (p1_result.to_string(), "Not Implemented".to_string());
}

fn parse_hand(hand_text: &str) -> Hand {
    
    //Extract the basic data
    let mut line_parts = hand_text.split(' ');
    let cards = line_parts.nth(0).expect("Line parse failed").bytes();
    let bid = line_parts.nth(0).expect("Line parse failed").parse::<i64>().expect("Bid parse failed");

    //First, convert the cards to a more usable format
    let cards: Vec<u8> = cards.map(
        |card: u8|
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

    let hand_category = 
        if tuple_counts[3] == 1 {HandType::Five}
        else if tuple_counts[2] == 1 {HandType::Four}
        else if tuple_counts[1] == 1 {
            if tuple_counts[0] == 1 {
                HandType::FullHouse
            } else {
                HandType::Three
            }
        } else if tuple_counts[0] == 2 {
            HandType::TwoPair
        } else if tuple_counts[0] == 1 {
            HandType::Pair
        } else {
            HandType::HighCard
        };

    return Hand {cards: cards, bid: bid, category: hand_category};
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