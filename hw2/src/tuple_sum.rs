use std::ops::Add;
///функция tuple_sum принимает кортеж из двух целых чисел.
/// Возвращает целое число, равное сумме чисел во входном кортеже.
#[allow(dead_code)]
fn tuple_sum<T: Add<Output = T>>(pair: (T, T)) -> T {
    pair.0 + pair.1
}

#[cfg(test)]
mod tests {
    use super::tuple_sum;

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(tuple_sum((1_i64, 1_i64)), 2_i64);
        assert_eq!(tuple_sum((1_i64, -1_i64)), 0_i64);
        assert_eq!(tuple_sum((0_i64, 0_i64)), 0_i64);
        assert_eq!(tuple_sum((1_i64, 2_i64)), 3_i64);
    }
}
