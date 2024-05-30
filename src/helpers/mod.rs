#[cfg(test)]
pub mod test;

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
