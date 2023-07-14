pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

pub struct SignedCounterOps {
    counter: SignedCounter,
}
pub struct UnsignedCounterOps {
    counter: UnsignedCounter,
}

pub trait CounterOps<T> {
    fn default() -> Self;
    fn next(&mut self, increment: T) -> T;
    fn prev(&mut self, decrement: T) -> T;
}

impl CounterOps<SignedCounter> for SignedCounterOps {
    fn default() -> SignedCounterOps {
        SignedCounterOps { counter: 0 }
    }

    fn next(&mut self, increment: SignedCounter) -> SignedCounter {
        self.counter += increment;
        self.counter
    }
    fn prev(&mut self, decrement: SignedCounter) -> SignedCounter {
        self.counter -= decrement;
        self.counter
    }
}

impl CounterOps<UnsignedCounter> for UnsignedCounterOps {
    fn default() -> UnsignedCounterOps {
        UnsignedCounterOps { counter: 0 }
    }

    fn next(&mut self, increment: UnsignedCounter) -> UnsignedCounter {
        self.counter += increment;
        self.counter
    }
    fn prev(&mut self, decrement: UnsignedCounter) -> UnsignedCounter {
        self.counter = self.counter.saturating_sub(decrement);
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
        assert_eq!(counter.next(1), 1);
        assert_eq!(counter.next(-1), 0);
        assert_eq!(counter.next(2), 2);
    }

    #[test]
    fn test_next_unsigned() {
        let mut counter = UnsignedCounterOps::default();
        assert_eq!(counter.next(0), 0);
        assert_eq!(counter.next(1), 1);
        assert_eq!(counter.next(10), 11);
    }

    #[test]
    fn test_prev_signed() {
        let mut counter = SignedCounterOps::default();
        assert_eq!(counter.prev(1), -1);
        assert_eq!(counter.prev(1), -2);
        assert_eq!(counter.prev(-1), -1);
    }
}
