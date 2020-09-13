pub fn nth(mut n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut num: u32 = 1;

    while n > 0 {
        num += 2;
        if primes.iter().all(|p| num % p != 0){
            primes.push(num);
            n-=1
        }
    }

    *primes.last().unwrap()
}
