#![allow(dead_code)]

pub type SignedCounter = isize;
pub type UnsignedCounter = usize;
pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_signed_counter() {
        assert_eq!(default_signed_counter(), 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(default_unsigned_counter(), 0);
    }

    #[test]
    fn test_next_signed() {
        assert_eq!(next_signed(0), 1);
        assert_eq!(next_signed(-1), 0);
        assert_eq!(next_signed(1), 2);
    }

    #[test]
    fn test_next_unsigned() {
        assert_eq!(next_unsigned(0), 1);
        assert_eq!(next_unsigned(10), 11);
    }

    #[test]
    fn test_prev_signed() {
        assert_eq!(prev_signed(0), -1);
        assert_eq!(prev_signed(-1), -2);
        assert_eq!(prev_signed(1), 0);
    }
}
