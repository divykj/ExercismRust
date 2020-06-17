pub fn verse(n: u32) -> String {
    let bottles = |n, first| match (n, first) {
        (0, true) => "No more bottles".to_string(),
        (0, false) => "no more bottles".to_string(),
        (1, _) => "1 bottle".to_string(),
        (_, _) => format!("{} bottles", n),
    };

    format!(
        "{bottles} of beer on the wall, {bottles_again} of beer.\n{action}, {next_bottles} of beer on the wall.\n", 
        bottles = bottles(n, true),
        bottles_again = bottles(n, false),
        action = match n {
            0 => "Go to the store and buy some more",
            1 => "Take it down and pass it around",
            _ => "Take one down and pass it around"
        }.to_string(),
        next_bottles = bottles((n+99)%100, false)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
