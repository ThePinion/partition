use super::*;
use crate::helpers::test::{naive_sumset, verify_approximation};

fn verify_unrestricted_approximation(input: Vec<u32>, epsilon: f64) {
    let approximation = approximate_sumset(&input, epsilon);
    let additive_error =
        (epsilon * input.iter().copied().map(u64::from).sum::<u64>() as f64) as u64 / 4;
    verify_approximation(
        &approximation,
        &naive_sumset(&input.iter().copied().map(u64::from).collect::<Vec<u64>>()),
        0.0,
        additive_error,
    );
}

#[test]
fn test_unrestricted_approximation() {
    let input = [
        1001, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 1000, 1001, 1002, 1003, 5,
    ]
    .to_vec();
    let epsilon = 0.01;
    verify_unrestricted_approximation(input, epsilon)
}

#[test]
fn test_unrestricted_approximation_u32_max() {
    let input = (0..10)
        .into_iter()
        .map(|x| u32::MAX / 3 * 2 as u32 - x * x * x)
        .collect();
    let epsilon = 0.1;
    verify_unrestricted_approximation(input, epsilon)
}

#[test]
fn test_unrestricted_approximation_large() {
    let input = [
        1001, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 1000, 1001, 1002, 1003, 5,
    ]
    .repeat(100);
    let epsilon = 0.01;
    let approximation = approximate_sumset(&input, epsilon);
    assert!(approximation.len() >= input.len())
}

use proptest::prelude::*;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

proptest! {
    #[test]
    fn test_add(a in 0..1000i32, b in 0..1000i32) {
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
    }
}
