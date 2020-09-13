pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 { return vec![] }

    let mut result: Vec<u64> = vec![2];
    (3..=upper_bound).step_by(2).for_each(|n|
        if result.clone().into_iter().all(|p| n % p != 0) { result.push(n) });

    result
}
