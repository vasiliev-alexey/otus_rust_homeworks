use std::ops::Mul;
/// функция double_float64 принимает 32-х битное число с плавающей точкой
/// и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
#[allow(dead_code)]
fn double_float64(input: f32) -> f64 {
    input.mul(2.0) as f64
}

#[cfg(test)]
mod tests {
    use super::double_float64;

    #[test]
    fn test_double_float64() {
        assert_eq!(double_float64(1_f32), 2_f64);
        assert_eq!(double_float64(0_f32), 0_f64);
    }
}
