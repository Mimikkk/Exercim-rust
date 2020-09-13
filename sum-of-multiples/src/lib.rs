use std::collections::HashSet;

pub fn a_sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.into_iter().filter(|x| **x != 0)
        .map(|x| (0..limit).step_by(*x as usize).collect::<HashSet<u32>>())
        .fold(HashSet::<u32>::new(), |mut acc, x|
            { acc.extend(&x); acc }).into_iter().sum()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|&n| factors.iter().any(|&f| f != 0 && n % f == 0)).sum()
}