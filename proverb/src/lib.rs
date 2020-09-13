pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {return String::new()}
    let mut result: String = String::new();

    (1..list.len()).for_each(|i|result.push_str(
            format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]).as_str()));

    result.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    return result;
}
