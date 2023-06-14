use std::ops::Add;
/// функция int_plus_float_to_float принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой.
/// Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
#[allow(dead_code)]
fn int_plus_float_to_float(int_param: u32, float_param: f32) -> f64 {
    float_param.add(int_param as f32) as f64
}

#[cfg(test)]
mod tests {
    use super::int_plus_float_to_float;

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(int_plus_float_to_float(1_u32, 2_f32), 3_f64);
        assert_eq!(int_plus_float_to_float(0_u32, 0_f32), 0_f64);
        assert_eq!(int_plus_float_to_float(1_u32, 0_f32), 1_f64);
        assert_eq!(int_plus_float_to_float(1_u32, -1_f32), 0_f64);
    }
}
