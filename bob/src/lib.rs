use regex::Regex;

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let re = Regex::new("[a-zA-Z]").unwrap();

    let question: &str = "Sure.";
    let yell: &str = "Whoa, chill out!";
    let yell_question: &str = "Calm down, I know what I'm doing!";
    let nothing: &str = "Fine. Be that way!";
    let anything_else: &str = "Whatever.";

    if message.is_empty() {return nothing};
    let is_question: bool = message.ends_with("?");
    let is_yell: bool = message.to_uppercase() == message && re.is_match(message);
    if is_question && is_yell {return yell_question}
    if is_question {return question}
    if is_yell {return yell}
    return anything_else
}
