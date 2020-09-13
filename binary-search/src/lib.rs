pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None }

    let mut a: usize = 0;
    let mut b: usize = array.len() - 1;
    let mut i: usize;

    while a <= b {
        i = (a+b)/2;
        if array[i] == key {return Some(i)}

        if array[i] < key { a = i + 1 }
        else if i == 0 {return None}
        else { b = i - 1 }
    }
    None
}
