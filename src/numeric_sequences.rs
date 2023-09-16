pub mod catalan;
pub mod factorial;
pub mod fibonacci;

#[cfg(feature = "alloc")]
pub mod primes;

pub use catalan::Catalan;
pub use factorial::Factorial;
pub use fibonacci::Fibonacci;

#[cfg(feature = "alloc")]
pub use primes::Primes;
