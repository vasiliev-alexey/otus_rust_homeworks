#[allow(dead_code)]
fn f() -> i32 {
    println!("fo call");
    1
}
fn fo() -> i32 {
    println!("fo0 call");
    22222
}
fn foo() -> i32 {
    println!("fooo call");
    3
}
fn fooo() -> i32 {
    println!("fooo call");
    4
}

fn test() -> i32 {
    println!("fooo call");
    -1
}

// fn res() -> (i32, i32) {
//     (fo(), fooo())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use task2::macro2;

    #[test]
    fn test_2() {
        // let res22 = macro2!("foo", "fo", "fooo");
        let (fo_result, fooo_result) = macro2!("fo", "foo", "fooo");
        // println!("res: {}", res22.0);
        println!("res: {}", fo_result);
        println!("res: {}", fooo_result);

        //   let (fo_result, fooo_result) = my_macro!(""fo"", ""foo"", ""fooo"");
    }
}
