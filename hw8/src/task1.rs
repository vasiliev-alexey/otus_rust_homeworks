#[allow(unused_macros)]
macro_rules! my_macro {

        () => (
        ()
    );

    ($($func:ident),*) => {
        (
            $(
                $func()
            ),*
        )
    };
}

#[cfg(test)]
mod tests {

    fn foo() -> i32 {
        1
    }
    fn bar() -> usize {
        2_usize
    }
    fn baz() -> String {
        String::from("baz")
    }

    #[test]
    fn test_macros_with_args() {
        let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);
        assert_eq!(foo_result, 1);
        assert_eq!(bar_result, 2_usize);
        assert_eq!(baz_result, "baz");
    }

    #[test]
    fn test_macros_without_args() {
        let empty_slice = my_macro!();
        assert_eq!(empty_slice, ());
    }
}
