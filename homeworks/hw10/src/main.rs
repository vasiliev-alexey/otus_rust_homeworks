use hw10::{Cat, Pet};

fn main() {
    let mut tom = Cat::new("Tom", 3);
    let behemoth = Cat::new("Behemoth", 100);
    println!("{tom}");

    tom.increment_age();
    println!("Updated cat {} age: {}", tom.name, tom.age);

    let pet_one: Pet = tom.into();
    if let Pet::Cat(cat) = pet_one {
        println!("Cast back to Cat: {:?}", cat);
    }

    let pet_two: Pet = behemoth.into();
    let behemoth: Option<Cat> = pet_two.into();
    if let Some(cat) = behemoth {
        println!("Cast back to Cat: {:?}", cat);
    }

    let leopold = Cat::new("Leopold", 2);
    let leopold_old = leopold + 1;
    println!("{leopold_old:?}");

    let mut garfield = Cat::new("Garfield", 5);
    garfield += 2;
    println!("{:?}", garfield.age);
}
