pub fn verify_approximation(
    approximation: &[u64],
    expected: &[u64],
    delta_mul: f64,
    delta_add: u64,
) {
    for b in expected.iter() {
        verify_element_in_approximation(approximation, *b, delta_mul, delta_add);
    }
    for a in approximation.iter() {
        assert!(
            expected
                .iter()
                .filter(|&b| a <= b)
                .any(|b| ((1f64 - delta_mul) * (*b as f64)) as u64 <= *a + delta_add),
            "{:?} (1-{:?}, {:?})-approximation not found in (expected) {:?}",
            a,
            delta_mul,
            delta_add,
            expected
        );
    }
}

pub fn verify_element_in_approximation(
    approximation: &[u64],
    expected_element: u64,
    delta_mul: f64,
    delta_add: u64,
) {
    assert!(
        approximation
            .iter()
            .filter(|&a| *a <= expected_element)
            .any(|a| ((1f64 - delta_mul) * (expected_element as f64)) as u64 <= *a + delta_add),
        "{:?} (actual) not found in approximation: {:?}",
        expected_element,
        approximation
    );
}

use std::collections::HashSet;

pub fn naive_sumset(vec: &[u64]) -> Vec<u64> {
    let mut result = HashSet::new();
    generate_sumset(vec, 0, 0, &mut result);
    if !vec.contains(&0) {
        result.remove(&0);
    }
    result.into_iter().collect()
}

fn generate_sumset(vec: &[u64], index: usize, current_sum: u64, result: &mut HashSet<u64>) {
    if index == vec.len() {
        result.insert(current_sum);
        return;
    }
    generate_sumset(vec, index + 1, current_sum + vec[index], result);
    generate_sumset(vec, index + 1, current_sum, result);
}
