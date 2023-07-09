use ops::counter_ops::{CounterOps, UnsignedCounterOps};
use ops::vector_ops::{Vec3, Vec3Ops};
mod ops;

fn main() {
    let mut counter = UnsignedCounterOps::default();
    println!("{}", counter.next(10));

    let mut vector: Vec3 = Vec3Ops::default_vec3();
    println!("{:?}", vector.vec3_vector_sum([1, 2, 3]));

    println!("hw5 done");
}
