use crate::helpers::test::{naive_sumset, verify_approximation};

use super::approximate_sumset;

fn verify_unrestricted_approximation(input: Vec<u16>, epsilon: f64) {
    let approximation = approximate_sumset(&input, epsilon);
    let additive_error =
        (epsilon * input.iter().copied().map(u64::from).sum::<u64>() as f64) as u64 / 4;
    verify_approximation(
        &approximation,
        &[
            naive_sumset(&input.iter().copied().map(u64::from).collect::<Vec<u64>>()),
            vec![0],
        ]
        .concat(),
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
        .map(|x| u16::MAX as u16 - x * x * x)
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

#[cfg(feature = "use-proptest")]
mod proptest_tests {
    use super::*;
    use crate::helpers::test::verify_element_in_approximation;
    use proptest::prelude::*;

    const MAX_LENGTH: usize = 100;

    proptest! {
        #[test]
        fn test_unrestricted_approximation_proptest(
            a in prop::collection::vec(0..u16::MAX, 0..=MAX_LENGTH/2),
            b in prop::collection::vec(0..u16::MAX, 0..=MAX_LENGTH/2),
            eps_inv in 2..100,
        ) {
            let expected_element = a.iter().copied().map(u64::from).sum();
            let sigma: u64 = expected_element + b.iter().copied().map(u64::from).sum::<u64>();
            let epsilon = 1.0 / eps_inv as f64;
            let additive_error =
                (epsilon * sigma as f64) as u64 / 4;
            let approximation = approximate_sumset(&[a, b].concat(), epsilon);
            verify_element_in_approximation(&approximation, expected_element, 0.0, additive_error);
        }
    }
}
