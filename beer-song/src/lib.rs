// This exercise is dull.

pub fn lower_bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => format!("{} bottle", n),
        _ => format!("{} bottles", n)
    }
}
pub fn upper_bottles(n: u32) -> String {
    match n {
        0 => "No more bottles".to_string(),
        1 => format!("{} bottle", n),
        _ => format!("{} bottles", n)
    }
}

pub fn verse(bottles: u32) -> String {
    match bottles {
        0 => format!("{} of beer on the wall, {} of beer.\nGo to the store and buy some more, {} of beer on the wall.\n", upper_bottles(bottles), lower_bottles(bottles), lower_bottles(99)),
        1 => format!("{} of beer on the wall, {} of beer.\nTake it down and pass it around, {} of beer on the wall.\n", upper_bottles(bottles), lower_bottles(bottles), lower_bottles(bottles-1)),
        _ => format!("{} of beer on the wall, {} of beer.\nTake one down and pass it around, {} of beer on the wall.\n", upper_bottles(bottles), lower_bottles(bottles), lower_bottles(bottles-1)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let verse_list: Vec<String> = (end..=start).rev().map(verse).collect();
    verse_list.join("\n")
}