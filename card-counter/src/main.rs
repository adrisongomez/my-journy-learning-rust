const NUMBERS: [char; 14] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'j', 'q', 'k', 'a',
];
const FAMILY: [char; 4] = ['c', 'd', 'p', 't'];

// fn get_count_of_full_deck(cards: Vec<&str>) -> u32 {
//     return 5u32;
// }

fn main() {
    let mut full_deck: Vec<String> = Vec::new();
    for f in FAMILY {
        for n in NUMBERS {
            full_deck.push(format!("{n}{f}"))
        }
    }
    dbg!(full_deck);
    // get_count_of_full_deck(Vec::new());
}
