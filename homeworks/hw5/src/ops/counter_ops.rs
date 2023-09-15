pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

#[derive(Default)]
pub struct SignedCounterOps {
    counter: SignedCounter,
}
#[derive(Default)]
pub struct UnsignedCounterOps {
    counter: UnsignedCounter,
}

pub trait CounterOps {
    type Output;
    fn next(&mut self) -> Self::Output;
    fn prev(&mut self) -> Self::Output;
}

impl CounterOps for SignedCounterOps {
    type Output = SignedCounter;
    fn next(&mut self) -> Self::Output {
        self.counter += 1;
        self.counter
    }

    fn prev(&mut self) -> Self::Output {
        self.counter -= 1;
        self.counter
    }
}

impl CounterOps for UnsignedCounterOps {
    type Output = UnsignedCounter;
    fn next(&mut self) -> Self::Output {
        self.counter += 1;
        self.counter
    }

    fn prev(&mut self) -> Self::Output {
        self.counter = self.counter.saturating_sub(1);
        self.counter
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_default_signed_counter() {
        let counter = SignedCounterOps::default();
        assert_eq!(counter.counter, 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        let counter = UnsignedCounterOps::default();
        assert_eq!(counter.counter, 0);
    }

    #[test]
    fn test_next_signed() {
        let mut counter = SignedCounterOps::default();
        assert_eq!(counter.next(), 1);
        assert_eq!(counter.next(), 2);
        assert_eq!(counter.next(), 3);
    }

    #[test]
    fn test_next_unsigned() {
        let mut counter = UnsignedCounterOps::default();
        assert_eq!(counter.next(), 1);
        assert_eq!(counter.next(), 2);
        assert_eq!(counter.next(), 3);
    }

    #[test]
    fn test_prev_signed() {
        let mut counter = SignedCounterOps::default();
        assert_eq!(counter.next(), 1);
        assert_eq!(counter.next(), 2);
        assert_eq!(counter.prev(), 1);
        assert_eq!(counter.prev(), 0);
    }
}
