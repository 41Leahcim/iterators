use core::ops::{self, AddAssign, Sub};

pub struct Range<T> {
    start: T,
    end: T,
    step: T,
    value: T,
}

impl<T: PartialOrd + Default + Copy> Range<T> {
    /// This new function assumes that default returns the "zero" value
    ///
    /// # Panics
    /// If the step doesn't move the iterator towards the end.
    #[inline]
    pub fn new(start: T, end: T, step: T) -> Self {
        let zero = T::default();
        assert!(
            step != zero && ((start < end && step > zero) || (start > end && step < zero)),
            "Step doesn't bring the value closer to the end"
        );
        Self {
            start,
            end,
            step,
            value: start,
        }
    }
}

impl<T: PartialOrd + Copy + AddAssign> Iterator for Range<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;
        ((self.start <= self.end && self.value <= self.end)
            || (self.start >= self.end && self.value >= self.end))
            .then(|| {
                self.value += self.step;
                value
            })
    }
}

impl<T: PartialOrd + Sub<Output = T> + Copy + TryFrom<u8>> TryFrom<ops::Range<T>> for Range<T> {
    type Error = <T as TryFrom<u8>>::Error;
    #[inline]
    fn try_from(value: ops::Range<T>) -> Result<Self, Self::Error> {
        let one = T::try_from(1)?;
        Ok(Self {
            start: value.start,
            end: value.end - one,
            step: one,
            value: value.start,
        })
    }
}

impl<T: PartialOrd + Copy + TryFrom<u8>> TryFrom<ops::RangeInclusive<T>> for Range<T> {
    type Error = <T as TryFrom<u8>>::Error;
    #[inline]
    fn try_from(value: ops::RangeInclusive<T>) -> Result<Self, Self::Error> {
        let one: T = T::try_from(1)?;
        Ok(Self {
            start: *value.start(),
            end: *value.end(),
            step: one,
            value: *value.start(),
        })
    }
}
