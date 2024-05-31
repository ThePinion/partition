use partition::approximation::interval::SumsetIntervalApproximation;

fn main() {
    dbg!(SumsetIntervalApproximation::new(10, 0.1).approximate(&[10, 12, 13, 14]));
}
