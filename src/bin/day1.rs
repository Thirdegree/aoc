
const REPLACES: [(&str, char);9] = [
    ("one",   '1'),
    ("two",   '2'),
    ("three", '3'),
    ("four",  '4'),
    ("five",  '5'),
    ("six",   '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine",  '9'),
];

pub fn main() {
    let mut inp = include_str!("../day1/input.txt").to_string();
    // This is bad and messy, but it does work so hey
    let mut to_ins: Vec<_> = vec![];
    for (from, to) in REPLACES {
        // figure out all the places we need to insert a digit
        to_ins.extend(inp.match_indices(from).map(|p| (p.0, to)));
    }
    // Sort so that the order of index addition logic works below
    to_ins.sort_by_key(|p| p.0);
    for (n, (idx, to)) in to_ins.iter().enumerate() {
        // Inset the digit, also remembering to inc the insert index to account for the other
        // digits we've already inserted.
        inp.insert(idx + n, *to);
    }

    // Do the actual logic pls and thx
    let result: u32 = inp.lines().map(|line| {
        // eprintln!("{line}");
        let mut citer = line.chars().filter(char::is_ascii_digit);
        let dig1 = citer.next().unwrap();
        let dig2 = citer.last().unwrap_or(dig1);
        format!("{dig1}{dig2}").parse::<u32>().unwrap()
    }).sum();
    println!("Day1 result: {result}");
}
