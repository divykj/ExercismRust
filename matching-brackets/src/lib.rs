pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for ch in string.chars() {
        match ch {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '}' | ']' | ')' if stack.pop() != Some(ch) => return false,
            _ => (),
        }
    }
    stack.len() == 0
}
