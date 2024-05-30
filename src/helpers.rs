#[inline]
pub const fn ceil_div(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

pub struct PowerOfTwoIterator {
    current: u64,
    limit: u64,
}

impl PowerOfTwoIterator {
    pub fn new(start: u64, limit: u64) -> Self {
        // Find the smallest power of 2 greater than or equal to start
        let mut current = 1;
        while current < start {
            current *= 2;
        }
        PowerOfTwoIterator { current, limit }
    }
}

impl Iterator for PowerOfTwoIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.limit {
            None
        } else {
            let result = self.current;
            self.current *= 2;
            Some(result)
        }
    }
}

#[cfg(test)]
pub fn verify_approximation(
    approximation: &[u64],
    expected: &[u64],
    delta_mul: f64,
    delta_add: u64,
) {
    for b in expected.iter() {
        assert!(
            approximation
                .iter()
                .filter(|&a| a <= b)
                .find(|&a| ((1f64 - delta_mul) * (*b as f64)) as u64 <= *a + delta_add)
                .is_some(),
            "{:?} (actual) not found in approximation: {:?}",
            b,
            approximation
        );
    }
    for a in approximation.iter() {
        assert!(
            expected
                .iter()
                .filter(|&b| a <= b)
                .find(|&b| ((1f64 - delta_mul) * (*b as f64)) as u64 <= *a + delta_add)
                .is_some(),
            "{:?} (1-{:?}, {:?})-approximation not found in (expected) {:?}",
            a,
            delta_mul,
            delta_add,
            expected
        );
    }
}
