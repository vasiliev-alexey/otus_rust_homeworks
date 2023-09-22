use std::fmt::Display;
use std::ops::{Add, AddAssign};
#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u32,
}

impl Cat {
    pub fn new(name: &str, age: u32) -> Cat {
        Cat {
            name: String::from(name),
            age,
        }
    }

    #[allow(dead_code)]
    fn name(&self) -> &str {
        &self.name
    }

    pub fn increment_age(&mut self) {
        self.age += 1;
    }
}

impl Clone for Cat {
    fn clone(&self) -> Cat {
        Cat {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cat: {} - {} years old", self.name, self.age)
    }
}
#[derive(Debug)]
pub struct Dog {}

#[derive(Debug)]
pub enum Pet {
    Dog(Dog),
    Cat(Cat),
}

impl From<Cat> for Pet {
    fn from(cat: Cat) -> Self {
        Pet::Cat(cat)
    }
}

impl From<Pet> for Option<Cat> {
    fn from(pet: Pet) -> Self {
        match pet {
            Pet::Cat(cat) => Some(cat),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CatCastError;
impl TryFrom<Pet> for Cat {
    type Error = CatCastError;

    fn try_from(pet: Pet) -> Result<Self, CatCastError> {
        match pet {
            Pet::Cat(cat) => Ok(cat),
            _ => Err(CatCastError),
        }
    }
}

impl Add<u32> for Cat {
    type Output = Cat;

    fn add(self, rhs: u32) -> Cat {
        Cat {
            name: self.name,
            age: self.age + rhs,
        }
    }
}

impl AddAssign<u32> for Cat {
    fn add_assign(&mut self, rhs: u32) {
        self.age += rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::Pet::Dog;

    #[test]
    fn test_cat_new() {
        let cat = Cat::new("Gav", 1);
        assert_eq!(cat.name(), "Gav");
        assert_eq!(cat.age, 1);
    }

    #[test]
    fn test_cat_clone() {
        let cat1 = Cat::new("Gav", 1);
        let cat2 = cat1.clone();
        assert_eq!(cat1.name, cat2.name);
        assert_eq!(cat1.age, cat2.age);
    }

    #[test]
    fn test_cat_name() {
        let cat = Cat::new("Gav", 3);
        assert_eq!(cat.name(), "Gav");
    }

    #[test]
    fn test_cat_increment_age() {
        let mut cat = Cat::new("Gav", 1);
        cat.increment_age();
        assert_eq!(cat.age, 2);
    }

    #[test]
    fn test_cat_display() {
        let cat = Cat::new("Gav", 1);
        assert_eq!(format!("{cat}"), "Cat: Gav - 1 years old");
    }

    #[test]
    fn test_cat_into_pet() {
        let cat = Cat::new("Gav", 1);
        let pet: Pet = cat.into();
        if let Pet::Cat(c) = pet {
            assert_eq!(c.name(), "Gav");
            assert_eq!(c.age, 1);
        } else {
            panic!("Expected Pet::Cat, but got {pet:?}");
        }
    }

    #[test]
    fn test_try_cat_into_pet() {
        let pet = Pet::Cat(Cat::new("Gav", 1));

        let cat = Cat::try_from(pet);

        if let Ok(cat) = cat {
            assert_eq!(cat.name, "Gav");
            assert_eq!(cat.age, 1);
        } else {
            panic!("Expected Pet::Cat, but got {cat:?}");
        }
    }

    #[test]
    fn test_pet_into_cat() {
        let pet = Pet::Cat(Cat::new("Gav", 1));
        let cat: Option<Cat> = pet.into();
        if let Some(c) = cat {
            assert_eq!(c.name(), "Gav");
            assert_eq!(c.age, 1);
        } else {
            panic!("Expected Some(Cat), but got None");
        }
    }

    #[test]
    #[should_panic(expected = "Expected Some(Cat), but got None")]
    fn test_try_dog_into_cat() {
        let dog = Dog {};
        let pet = Pet::Dog(dog);
        let cat: Option<Cat> = pet.into();
        if cat.is_some() {
            panic!("Unexpected cast to Cat");
        } else {
            panic!("Expected Some(Cat), but got None");
        }
    }

    #[test]
    fn test_cat_add() {
        let cat1 = Cat::new("Gav", 1);
        let cat2 = cat1 + 2;
        assert_eq!(cat2.age, 3);
    }

    #[test]
    fn test_cat_add_assign() {
        let mut cat = Cat::new("Gav", 1);
        cat += 2;
        assert_eq!(cat.age, 3);
    }
}
