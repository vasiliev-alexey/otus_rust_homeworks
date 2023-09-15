fn main() {
    const COUNT: usize = 100;
    (0..COUNT)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        })
        .for_each(|x| println!("{x}"));
}
