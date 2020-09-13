pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut p: u64 = 3;


    while n % 2 == 0 { result.push(2); n /= 2; }
    while n != 1 {
        while n % p == 0 { result.push(p); n /= p; }
        p+=2;
    }

    result
}
