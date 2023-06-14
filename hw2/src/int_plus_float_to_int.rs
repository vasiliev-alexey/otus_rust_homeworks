use std::ops::Add;
/// функция int_plus_float_to_int принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой.
/// Возвращает 64-х битное целое беззнаковое число, равное сумме входных.

#[allow(dead_code)]
fn int_plus_float_to_int(int_param: u32, float_param: f32) -> u64 {
    float_param.add(int_param as f32) as u64
}

#[cfg(test)]
mod tests {
    use super::int_plus_float_to_int;

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(int_plus_float_to_int(1_u32, 2_f32), 3_u64);
        assert_eq!(int_plus_float_to_int(0_u32, 0_f32), 0_u64);
        assert_eq!(int_plus_float_to_int(1_u32, 0_f32), 1_u64);
        assert_eq!(int_plus_float_to_int(1_u32, -1_f32), 0_u64);
    }
}
