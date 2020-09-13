pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();

    for c in string.chars() {
        match c {
            '(' => brackets.push(')'),
            '{' => brackets.push('}'),
            '[' => brackets.push(']'),
            ')' | '}' | ']' => {
                if !brackets.is_empty() && &c == brackets.last().unwrap() {
                    brackets.pop();
                } else {
                    brackets.push('*');
                }
            }
            _ => {},
        }
    }
    brackets.is_empty()
}