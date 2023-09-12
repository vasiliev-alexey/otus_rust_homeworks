use std::ops::Mul;
/// функция double_float64 принимает 32-х битное число с плавающей точкой
/// и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
#[allow(dead_code)]
fn double_float64(input: f32) -> f64 {
    (input as f64).mul(2.0)
}

#[cfg(test)]
mod tests {
    use super::double_float64;

    #[test]
    fn test_double_float64() {
        assert_eq!(double_float64(1_f32), 2_f64);
        assert_eq!(double_float64(0_f32), 0_f64);
        assert_eq!(
            double_float64(f32::MAX),
            (f32::MAX as f64) + (f32::MAX as f64)
        );
    }
}
