use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let re = Regex::new(r"[A-Z]+[a-z']*|[a-z]+").unwrap();

    re.captures_iter(phrase)
        .map(|cap|
            cap.get(0).map_or(' ', |m| m.as_str().chars().next().unwrap()))
        .collect::<String>().to_uppercase()
}