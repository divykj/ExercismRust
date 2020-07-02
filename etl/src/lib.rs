use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut scores = BTreeMap::<char, i32>::new();
    for (score, letters) in h.iter() {
        for letter in letters {
            scores.insert(letter.to_ascii_lowercase(), *score);
        }
    }

    scores
}
