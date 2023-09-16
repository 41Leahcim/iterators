#[cfg(feature = "alloc")]
use iterators::numeric_sequences::Primes;
use iterators::{
    numeric_sequences::{Catalan, Factorial, Fibonacci},
    range::{Range, RangeUsize},
};
use std::{hint::black_box, time::Instant};

const CYCLES: usize = 1_000_000_000;

fn test_iterator(iter: impl Iterator, name: &str) {
    let start = Instant::now();
    for _ in iter {
        black_box(())
    }
    println!("{name}: {}", start.elapsed().as_secs_f64());
}

fn main() {
    test_iterator(Range::new(0, CYCLES, 1), "Range");
    test_iterator(Range::from(0..CYCLES + 1), "Range from std Range");
    test_iterator(Range::from(0..=CYCLES), "Range from std RangeInclusive");
    test_iterator(RangeUsize::<0, CYCLES, 1>::default(), "RangeUsize");
    test_iterator(0..=CYCLES, "Std RangeInclusive");
    test_iterator(Fibonacci::default().cycle().take(CYCLES), "Fibonacci");
    test_iterator(Factorial::default().take(CYCLES), "Factorial");
    test_iterator(Catalan::default().cycle().take(CYCLES), "Catalan");
    #[cfg(feature = "alloc")]
    test_iterator(Primes::default().cycle().take(CYCLES), "Prime generator")
}
