#[cfg(feature = "alloc")]
use iterators::numeric_sequences::Primes;
use iterators::{
    numeric_sequences::{Catalan, Factorial, Fibonacci},
    range::{Range, RangeUsize},
};
use std::{hint::black_box, thread, time::Instant};

const CYCLES: usize = 1_000_000_000;

fn test_iterator(iter: impl Iterator, name: &str) {
    let start = Instant::now();
    for _ in iter {
        black_box(())
    }
    println!("{name}: {}", start.elapsed().as_secs_f64());
}

fn main() {
    thread::scope(|scope| {
        scope.spawn(|| test_iterator(Range::new(0, CYCLES, 1), "Range"));
        scope.spawn(|| test_iterator(Range::from(0..CYCLES + 1), "Range from std Range"));
        scope.spawn(|| test_iterator(Range::from(0..=CYCLES), "Range from std RangeInclusive"));
        scope.spawn(|| test_iterator(RangeUsize::<0, CYCLES, 1>::default(), "RangeUsize"));
        scope.spawn(|| test_iterator(0..=CYCLES, "Std RangeInclusive"));
        scope.spawn(|| test_iterator(Fibonacci::default().cycle().take(CYCLES), "Fibonacci"));
        scope.spawn(|| test_iterator(Factorial::default().take(CYCLES), "Factorial"));
        #[cfg(feature = "alloc")]
        scope.spawn(|| test_iterator(Primes::default().cycle().take(CYCLES), "Prime generator"));
        test_iterator(Catalan::default().cycle().take(CYCLES), "Catalan");
    });
}
