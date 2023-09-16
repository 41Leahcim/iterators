use core::{
    fmt::Debug,
    ops::{AddAssign, Sub},
};

pub struct RangeUsize<const START: usize, const END: usize, const STEP: usize>(usize);

impl<const START: usize, const END: usize, const STEP: usize> Default
    for RangeUsize<START, END, STEP>
{
    fn default() -> Self {
        Self(START)
    }
}

impl<const START: usize, const END: usize, const STEP: usize> Iterator
    for RangeUsize<START, END, STEP>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0;
        if value > END {
            None
        } else {
            self.0 += STEP;
            Some(value)
        }
    }
}

pub struct Range<T> {
    start: T,
    end: T,
    step: T,
    value: T,
}

impl<T: PartialOrd + Default + Copy> Range<T> {
    /// This new function assumes that default returns the "zero" value
    pub fn new(start: T, end: T, step: T) -> Self {
        let zero = T::default();
        assert!(step != zero && ((start < end && step > zero) || (start > end && step < zero)));
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

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;
        if (self.start <= self.end && self.value <= self.end)
            || (self.start >= self.end && self.value >= self.end)
        {
            self.value += self.step;
            Some(value)
        } else {
            None
        }
    }
}

impl<T: PartialOrd + Sub<Output = T> + Copy + TryFrom<u8>> From<core::ops::Range<T>> for Range<T>
where
    <T as TryFrom<u8>>::Error: Debug,
{
    fn from(value: core::ops::Range<T>) -> Self {
        let one: T = 1.try_into().unwrap();
        Self {
            start: value.start,
            end: value.end - one,
            step: one,
            value: value.start,
        }
    }
}

impl<T: PartialOrd + Copy + TryFrom<u8>> From<core::ops::RangeInclusive<T>> for Range<T>
where
    <T as TryFrom<u8>>::Error: Debug,
{
    fn from(value: core::ops::RangeInclusive<T>) -> Self {
        let one: T = 1.try_into().unwrap();
        Self {
            start: *value.start(),
            end: *value.end(),
            step: one,
            value: *value.start(),
        }
    }
}
