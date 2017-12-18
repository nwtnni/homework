#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::mat_mult;
use problem3::sieve;
use problem4::{hanoi, Peg};

#[test]
fn test_sieve() {
    assert_eq!(sieve(50), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47])
}

#[test]
fn test_hanoi() {
    let result = [(Peg::A, Peg::B), (Peg::A, Peg::C), (Peg::B, Peg::C)];
    assert_eq!(hanoi(2, Peg::A, Peg::B, Peg::C), result)
}
