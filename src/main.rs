use partition::subset_sum::*;

fn main() {
    dbg!(subset_sum(&[1, 2], &[1, 100]));
    dbg!(subset_sum_2d(&[(1, 0), (2, 1)], &[(1, 10), (100, 20)]));
}
