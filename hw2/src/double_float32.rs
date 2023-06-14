use std::ops::Mul;
/// функция double_float32 принимает 32-х битное число с плавающей точкой
/// и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.
#[allow(dead_code)]
fn double_float32(input: f32) -> f32 {
    input.mul(2.0)
}

#[cfg(test)]
mod tests {
    use super::double_float32;

    #[test]
    fn test_double_float32() {
        assert_eq!(double_float32(1.6_f32), 3.2_f32);
        assert_eq!(double_float32(0_f32), 0_f32);
        assert_eq!(double_float32(-1.2_f32), -2.4_f32);
    }
}
