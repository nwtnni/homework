/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for i in slice {
        sum += i;
    }
    sum
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut unique = Vec::new();    
    for i in vs {
        if !unique.contains(i) {
            unique.push(*i) 
        }
    }
    unique
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut filtered = Vec::new();
    for i in vs {
        if pred(*i) {
            filtered.push(*i);
        }
    }
    filtered
}
