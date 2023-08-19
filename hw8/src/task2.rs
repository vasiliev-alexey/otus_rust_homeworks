use task2::gen_dummy_function;

gen_dummy_function!(0);
gen_dummy_function!(1);
gen_dummy_function!(2);
gen_dummy_function!(3);
gen_dummy_function!(4);
gen_dummy_function!(9);
#[cfg(test)]
mod tests {
    use super::*;
    use task2::even_len_name_func_invoke;

    #[test]
    fn test_2() {
        let (fo_result, fooo_result, foooooooooo_result) =
            even_len_name_func_invoke!("fo", "foo", "fooo", "fooooooooo");
        assert_eq!(fo_result, 2);
        assert_eq!(fooo_result, 4);
        assert_eq!(foooooooooo_result, 10);
    }
}
