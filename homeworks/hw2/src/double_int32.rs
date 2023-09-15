/// функция double_int32 принимает 32-х битное целое беззнаковое число
/// и возвращает 32-х битное целое беззнаковое число, равное удвоенному входному.

#[allow(dead_code)]
fn double_int32(input: u32) -> u32 {
    input << 1
}

#[cfg(test)]
mod tests {
    use super::double_int32;

    #[test]
    fn test_double_int32() {
        assert_eq!(double_int32(1_u32), 2_u32);
        assert_eq!(double_int32(0_u32), 0_u32);
    }
}
