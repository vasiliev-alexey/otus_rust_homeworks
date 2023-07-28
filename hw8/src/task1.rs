#[allow(unused_macros)]
macro_rules! my_macro {

        () => (
        &[]
    );

    ($($func:ident),*) => {
        (
            $(
                $func()
            ),*
        )
    };
}
#[allow(dead_code)]
fn foo() -> i32 {
    1
}
#[allow(dead_code)]
fn bar() -> usize {
    2_usize
}
#[allow(dead_code)]
fn baz() -> String {
    String::from("baz")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macros_with_args() {
        let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);
        println!("foo_result: {}", foo_result);
        println!("bar_result: {:?}", bar_result);
        println!("baz_result: {}", baz_result);
    }

    #[test]
    fn test_macros_without_args() {
        let empty_slice: &[i32] = my_macro!();
        let length = empty_slice.len();
        assert_eq!(length, 0);
    }
}
