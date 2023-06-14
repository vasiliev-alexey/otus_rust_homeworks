/// функция double_int64 принимает 32-х битное целое беззнаковое число
/// и возвращает 64-х битное целое беззнаковое число, равное удвоенному входному.
#[allow(dead_code)]
fn double_int64(i: u32) -> u64 {
    (i as u64) << 1
}

#[cfg(test)]
mod tests {
    use super::double_int64;

    #[test]
    fn test_double_int64() {
        assert_eq!(double_int64(1_u32), 2_u64);
        assert_eq!(double_int64(0_u32), 0_u64);
        assert_eq!(double_int64(4_294_967_295_u32), 8_589_934_590_u64);
    }
}
