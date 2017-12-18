/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut possible: Vec<u32> = (2..n).collect();

    for i in 2..n {
        if possible.contains(&i) {
            possible = possible
                .into_iter()
                .filter( |n| n < &(i*i) || n % i != 0 )
                .collect()
        }
    }
    possible
}
