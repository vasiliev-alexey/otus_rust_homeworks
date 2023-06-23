#![allow(dead_code)]
const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];

fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..VEC3_LEN {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_vector_sum() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        let c = vec3_vector_sum(a, b);
        assert_eq!(c[0], 5);
        assert_eq!(c[1], 7);
        assert_eq!(c[2], 9);
    }

    #[test]
    fn test_vec3_vector_sum_with_zero_vec() {
        let a = default_vec3();
        let b = [1, 2, 3];
        let c = vec3_vector_sum(a, b);
        assert_eq!(c[0], 1);
        assert_eq!(c[1], 2);
        assert_eq!(c[2], 3);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let a = [1, 2, 3];
        let b = [4, 4, 6];

        let c = vec3_scalar_sum(a, b);
        assert_eq!(c, 20);
    }

    #[test]
    fn test_vec3_scalar_sum_with_zero_vec() {
        let a = default_vec3();
        let b = [4, 4, 6];
        let c = vec3_scalar_sum(a, b);
        assert_eq!(c, 14);
    }
}
