/// функция array_sum принимает массив из трёх целых чисел.
/// Возвращает целое число, равное сумме чисел во входном массиве.
#[allow(dead_code)]
pub fn array_sum(x: [i64; 3]) -> i64 {
    x.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::array_sum;

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(array_sum([1_i64, 1_i64, 1_i64]), 3_i64);
        assert_eq!(array_sum([1_i64, 2_i64, 3_i64]), 6_i64);
        assert_eq!(array_sum([0_i64, 0_i64, 0_i64]), 0_i64);
        assert_eq!(array_sum([0_i64, 1_i64, 0_i64]), 1_i64);
    }
}
