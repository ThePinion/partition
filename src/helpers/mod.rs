use std::{collections::BTreeMap, ops::Add};

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

pub fn reduce_multiplicity<T>(set: &[T]) -> BTreeMap<T, usize>
where
    T: Copy + Ord + Add<Output = T>,
{
    let mut map = BTreeMap::new();
    let mut new_map = BTreeMap::new();
    for i in set {
        *map.entry(*i).or_insert(0_usize) += 1;
    }
    for (&key, &mult) in map.iter() {
        reduce_single_element(key, mult, &mut new_map);
    }
    new_map
}

// Rust std doesn't have a tree-like structure that supports indexing in log(n) time
// This workaround slightly deviates from the paper but is correct nonetheless
fn reduce_single_element<T>(number: T, mult: usize, map: &mut BTreeMap<T, usize>)
where
    T: Copy + Ord + Add<Output = T>,
{
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
    reduce_single_element(number + number, mult, map);
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

pub fn dynamic_programing_partition(set: &[u64]) -> u64 {
    let sum = set.iter().sum::<u64>() / 2;
    let mut dp = vec![vec![false; sum as usize + 1]; set.len() + 1];

    for i in 0..=set.len() {
        dp[i][0] = true;
    }

    for i in 1..=sum as usize {
        dp[0][i] = false;
    }

    for i in 1..=set.len() {
        for j in 1..=sum as usize {
            if j < set[i - 1] as usize {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - set[i - 1] as usize];
            }
        }
    }

    return dp[set.len()]
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x { Some(i as u64) } else { None })
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_multiplicity_1() {
        let set = vec![1, 2, 2, 2, 4, 4, 3, 3, 3, 1, 3, 3, 3, 3];
        let reduced = reduce_multiplicity(&set);
        assert_eq!(
            reduced
                .into_iter()
                .flat_map(|(key, mult)| std::iter::repeat(key).take(mult))
                .collect::<Vec<_>>(),
            vec![1, 1, 2, 3, 4, 6, 8, 12]
        );
    }
    #[test]
    fn test_reduce_multiplicity_2() {
        let set = vec![1, 1, 1, 2, 2, 4, 4, 8, 8, 16, 16, 32, 32, 64, 64, 128, 128];
        let reduced = reduce_multiplicity(&set);
        assert_eq!(
            reduced
                .into_iter()
                .flat_map(|(key, mult)| std::iter::repeat(key).take(mult))
                .collect::<Vec<_>>(),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256]
        );
    }

    #[test]
    fn test_dynamic_programing_partition() {
        assert_eq!(dynamic_programing_partition(&vec![1, 2, 3, 4, 5]), 7);

        assert_eq!(dynamic_programing_partition(&vec![1, 2, 3, 4, 5, 6]), 10);

        assert_eq!(dynamic_programing_partition(&vec![1, 2, 3, 4, 5, 6, 7]), 14);

        assert_eq!(dynamic_programing_partition(&vec![1, 2, 3, 4, 5, 6, 7, 8]), 18);

        assert_eq!(dynamic_programing_partition(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 22);

        assert_eq!(dynamic_programing_partition(&vec![2; 1000]), 1000);
    }
}
