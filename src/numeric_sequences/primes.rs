use core::fmt::Debug;

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Default, Clone)]
pub struct Primes {
    primes: Vec<u64>,
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.primes.len() {
            0 => {
                self.primes.push(2);
                Some(2)
            }
            1 => {
                self.primes.push(3);
                Some(3)
            }
            _ => {
                let mut last_number = *self.primes.last()?;
                while let Some(number) = last_number.checked_add(2) {
                    if self
                        .primes
                        .iter()
                        .find(|&&prime| number % prime == 0 || number * number > prime)
                        .is_some_and(|&prime| number % prime != 0)
                    {
                        self.primes.push(number);
                        return Some(number);
                    }
                    last_number = number;
                }
                None
            }
        }
    }
}
