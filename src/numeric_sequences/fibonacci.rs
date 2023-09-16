/// Fibonacci sequence iterator
#[derive(Debug, Clone, Copy)]
pub struct Fibonacci {
    current: Option<u128>,
    next: Option<u128>,
}

/// Only constructor, current is the next to return
impl Default for Fibonacci {
    fn default() -> Self {
        Self {
            current: Some(0),
            next: Some(1),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        // If current and next are still within bounds
        if let (Some(current), Some(next)) = (self.current, self.next) {
            // Set current to next
            self.current = Some(next);

            // Set next to some(current + next) or None if it's out of bounds
            self.next = current.checked_add(next);

            // Return Some(current)
            Some(current)
        } else if let Some(current) = self.current {
            // If only current is within bounds
            // Set current to next
            self.current = self.next;

            // Return Some(current)
            Some(current)
        } else {
            // Otherwise, return None
            None
        }
    }
}
