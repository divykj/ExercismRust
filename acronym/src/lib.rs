pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = Vec::<char>::new();
    let mut prev_ch: char = ' ';
    let delimiters = " -_";

    for ch in phrase.chars() {
        if ch.is_alphabetic()
            && (delimiters.chars().any(|delimiter| prev_ch == delimiter)
                || (ch.is_uppercase() && !prev_ch.is_uppercase()))
        {
            acronym.push(ch)
        }
        prev_ch = ch;
    }

    acronym.iter().collect::<String>().to_uppercase()
}
