/// Factorial sequence iterator
#[derive(Debug, Default, Clone)]
pub struct Factorial(u128);

impl Iterator for Factorial {
    type Item = u128;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Try to multiply the current value with itself, will be None if not available
        let value = self.0.checked_mul(self.0);

        // Increment the value
        self.0 += 1;

        // Return the result
        value
    }
}
