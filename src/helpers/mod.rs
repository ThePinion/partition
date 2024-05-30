use std::collections::BTreeMap;

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

pub fn reduce_multiplicity(set: &[u64]) -> Vec<u64> {
    let mut map = BTreeMap::new();
    let mut new_map = BTreeMap::new();
    for i in set {
        *map.entry(*i).or_insert(0_u64) += 1;
    }
    for (&key, &mult) in map.iter() {
        reduce_single_element(key, mult, &mut new_map);
    }
    new_map
        .into_iter()
        .flat_map(|(key, mult)| std::iter::repeat(key).take(mult as usize))
        .collect()
}

// Rust std doesn't have a tree-like structure that supports indexing in log(n) time
// This workaround slightly deviates from the paper but is correct nonetheless
fn reduce_single_element(number: u64, mult: u64, map: &mut BTreeMap<u64, u64>) {
    if mult == 0 {
        return;
    }
    let mult = mult + map.get(&number).copied().unwrap_or(0);
    if mult <= 2 {
        map.insert(number, mult);
        return;
    }
    if mult % 2 == 1 {
        map.insert(number, 1);
    } else {
        map.insert(number, 2);
    }
    let mult = (mult - 1) / 2;
    reduce_single_element(number * 2, mult, map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_multiplicity_1() {
        let set = vec![1, 2, 2, 2, 4, 4, 3, 3, 3, 1, 3, 3, 3, 3];
        let reduced = reduce_multiplicity(&set);
        assert_eq!(reduced, vec![1, 1, 2, 3, 4, 6, 8, 12]);
    }
    #[test]
    fn test_reduce_multiplicity_2() {
        let set = vec![1, 1, 1, 2, 2, 4, 4, 8, 8, 16, 16, 32, 32, 64, 64, 128, 128];
        let reduced = reduce_multiplicity(&set);
        assert_eq!(reduced, vec![1, 2, 4, 8, 16, 32, 64, 128, 256]);
    }
}
