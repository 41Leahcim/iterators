/// Catalan sequency iterator
#[derive(Debug, Clone)]
pub struct Catalan([u128; 70], usize);

impl Default for Catalan {
    #[inline]
    fn default() -> Self {
        // Allocate an empty vector
        Self([0; 70], 0)
    }
}

impl Iterator for Catalan {
    type Item = u128;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the vector is empty
        if self.1 == 0 {
            // Push 1 intpo the vector
            self.0[0] = 1;
            self.1 += 1;
            Some(1)
        } else {
            // Otherwise
            // Set current to 0
            let mut current: u128 = 0;

            let array = &self.0[..self.1];

            // Take a normal and a reversed iterator over the Vec
            for (&left, &right) in array.iter().zip(array.iter().rev()) {
                // Try to add the product of the current values from the iterators
                // to the current value
                if let Some(result) = current.checked_add(left * right) {
                    // Set the current value to the result if within bounds
                    current = result;
                } else {
                    // Return None if it's out of bounds
                    return None;
                }
            }

            // Push the result to the vector
            self.0[self.1] = current;
            self.1 += 1;

            // Return the result
            Some(current)
        }
    }
}
