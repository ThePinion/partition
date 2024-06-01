use std::{collections::HashMap, ops::Add};

use crate::{helpers::reduce_multiplicity, subset_sum::subset_sum};

pub mod additive_merge;
pub mod interval;
pub mod multiplicative_merge;

pub use additive_merge::AdditiveBoundedMerger;
pub use interval::{SumsetEpsilonAdditiveAproximation, SumsetIntervalApproximation};
pub use multiplicative_merge::MultiplicativeBoundedMerger;

pub fn approximate_sumset(input: &[u64], epsilon: f64) -> Vec<u64> {
    let epsilon = epsilon / 2f64;
    let n = input.len();
    let eps_prim = epsilon / ((n as f64 / epsilon).log2() + 1f64);
    let eps_div_eps_prim = (epsilon / eps_prim).ceil() as u64;
    let epsilon = eps_div_eps_prim as f64 * eps_prim;
    let eps_inv = (1.0 / epsilon).ceil() as u64;
    let eps_prim_inv = eps_inv * eps_div_eps_prim;
    let _epsilon = 1.0 / (eps_inv as f64);
    let sigma: u64 = input.iter().sum();
    let _t = sigma / 2;

    let base = (sigma as f64 / (100f64 * n as f64 * eps_inv as f64)).ceil() as u64;
    let y_set = input
        .iter()
        .map(|&i| i / base)
        .filter(|&x| x != 0)
        .collect::<Vec<_>>();
    let scale = (100 * eps_inv).div_ceil(*y_set.iter().min().unwrap());
    let y_set = y_set.into_iter().map(|x| x * scale).collect::<Vec<_>>();

    let sigma = sigma * scale;
    let _t = sigma * 2;

    dbg!(
        eps_inv,
        base,
        y_set.iter().map(|x| x * base / scale).collect::<Vec<_>>()
    );

    let z_range_start = 100 * eps_inv;

    let z_set = y_set
        .iter()
        .map(|&x| ElementApproximation::new(z_range_start, x))
        .collect::<Vec<_>>();

    let z_set_prim = reduce_multiplicity(&z_set);
    let mut partition: HashMap<(u32, bool), Vec<u64>> = HashMap::new();

    for (el, &mult) in z_set_prim.iter() {
        assert!(mult <= 2);
        if mult >= 1 {
            partition.entry((el.k, false)).or_default().push(el.z);
        }
        if mult == 2 {
            partition.entry((el.k, true)).or_default().push(el.z);
        }
    }

    dbg!(&partition);

    let eps_inv_for_approx = eps_prim_inv * 100;

    dbg!(eps_inv_for_approx, eps_prim_inv, eps_div_eps_prim);

    let base_2 = (eps_prim * sigma as f64 / 100_f64).ceil() as u64;

    let a_js = partition
        .into_iter()
        .map(|((k, _), v)| {
            for &i in &*v {
                assert!(z_range_start <= i && i < z_range_start * 2)
            }
            let scaled = v.iter().map(|&x| x * eps_div_eps_prim).collect::<Vec<_>>();
            dbg!(&v);
            

            SumsetEpsilonAdditiveAproximation::new(eps_inv_for_approx)
                .approximate(&scaled)
                .into_iter()
                .map(|x| x / eps_div_eps_prim * 2_u64.pow(k))
                .map(|x| x / base_2)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut merged = merge_approximations(&a_js);

    merged.sort();

    for el in &mut merged {
        *el = *el * base_2 / scale * base;
    }
    merged
}

pub fn merge_approximations(a_js: &[Vec<u64>]) -> Vec<u64> {
    if a_js.is_empty() {
        return vec![];
    } else if a_js.len() == 1 {
        return a_js[0].clone();
    }
    let (left, right) = a_js.split_at(a_js.len() / 2);
    let (left, right) = (merge_approximations(left), merge_approximations(right));

    [subset_sum(&left, &right), left, right].concat()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ElementApproximation {
    k: u32,
    z: u64,
}

impl ElementApproximation {
    pub fn new(range_start: u64, element: u64) -> Self {
        assert!(element >= range_start);
        let mut k = 0;
        let mut cur = 1;
        let range_end = range_start * 2;
        while element / cur >= range_end {
            k += 1;
            cur *= 2;
        }
        let z = element / cur;
        assert!(range_start <= z && z < range_end);
        ElementApproximation { k, z }
    }
}

impl Add for ElementApproximation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert!(self.z == other.z && self.k == other.k);
        ElementApproximation {
            k: self.k + 1,
            z: self.z,
        }
    }
}

impl From<ElementApproximation> for u64 {
    fn from(val: ElementApproximation) -> Self {
        val.z * 2u64.pow(val.k)
    }
}

#[test]
fn test_approximation() {
    use crate::helpers::test::{naive_sumset, verify_approximation};
    let input = [1001, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 1000, 1001, 1002, 1003, 5]
    .repeat(1);
    let epsilon = 0.01;
    let approximation = approximate_sumset(&input, epsilon);
    let additive_error = (epsilon * input.iter().sum::<u64>() as f64) as u64 / 50;
    verify_approximation(&approximation, &naive_sumset(&input), 0.0, additive_error);
}
