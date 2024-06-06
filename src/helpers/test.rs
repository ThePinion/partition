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

