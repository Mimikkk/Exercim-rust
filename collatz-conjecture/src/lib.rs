pub fn collatz(n: u64) -> Option<u64> {
    return match n <= 0 {
        true => None,
        false => Some(count_steps(n)),
    }
}

fn count_steps(mut n: u64) -> u64 {
    let mut step: u64 = 0;
    while n != 1 {
        match n % 2 == 0 {
            true => n /= 2,
            false => n = 3*n+1,
        }
        step+=1
    }
    step
}