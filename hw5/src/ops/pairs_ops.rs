type Pair = (i32, i32);

trait PairOps {
    fn default_pair() -> Self;
    fn pair_vector_sum(&mut self, b: Pair) -> &Pair;
    fn pair_scalar_sum(self, b: Pair) -> i32;
}

impl PairOps for Pair {
    fn default_pair() -> Pair {
        (0, 0)
    }

    fn pair_vector_sum(&mut self, b: Pair) -> &Pair {
        self.0 += b.0;
        self.1 += b.1;
        self
    }

    fn pair_scalar_sum(self, b: Pair) -> i32 {
        self.0 + self.1 + b.0 + b.1
    }
}

#[cfg(test)]
mod tests_pairs_ops {
    use super::*;

    #[test]
    fn test_default_pair() {
        let default_pair: Pair = PairOps::default_pair();
        assert_eq!(default_pair, (0, 0));
    }

    #[test]
    fn test_pair_vector_sum() {
        let mut pair: Pair = PairOps::default_pair();
        assert_eq!(pair.pair_vector_sum((3, 4)), &(3, 4));
        assert_eq!(pair.pair_vector_sum((0, 0)), &(3, 4));
        assert_eq!(pair.pair_vector_sum((1, -1)), &(4, 3));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let pair: Pair = PairOps::default_pair();
        assert_eq!(pair.pair_scalar_sum((3, 4)), 7);
        assert_eq!(pair.pair_scalar_sum((0, 0)), 0);
        assert_eq!(pair.pair_scalar_sum((-1, 1)), 0);
    }
}
