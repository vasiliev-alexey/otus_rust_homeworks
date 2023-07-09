const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];

pub trait Vec3Ops {
    fn default_vec3() -> Self;
    fn vec3_vector_sum(&mut self, b: Vec3) -> &Self;
    fn vec3_scalar_sum(self, b: Vec3) -> i32;
}

impl Vec3Ops for Vec3 {
    fn default_vec3() -> Vec3 {
        [0; 3]
    }

    fn vec3_vector_sum(&mut self, b: Vec3) -> &Self {
        for i in 0..VEC3_LEN {
            self[i] += b[i];
        }
        self
    }

    fn vec3_scalar_sum(self, b: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += self[i] + b[i];
        }
        c
    }
}

#[cfg(test)]
mod tests_vec_ops {
    use super::*;

    #[test]
    fn test_vec3_vector_sum() {
        let mut a: Vec3 = Vec3Ops::default_vec3();
        let b = [4, 5, 6];
        let c = a.vec3_vector_sum(b);
        assert_eq!(c[0], 4);
        assert_eq!(c[1], 5);
        assert_eq!(c[2], 6);
    }

    #[test]
    fn test_vec3_vector_sum_with_zero_vec() {
        let mut a: Vec3 = Vec3Ops::default_vec3();
        let b = [1, 2, 3];
        let c = a.vec3_vector_sum(b);
        assert_eq!(c[0], 1);
        assert_eq!(c[1], 2);
        assert_eq!(c[2], 3);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let mut a: Vec3 = Vec3Ops::default_vec3();
        let b = [1, 2, 3];
        let c = a.vec3_vector_sum(b);
        let b = [4, 4, 6];
        let c = c.vec3_scalar_sum(b);
        assert_eq!(c, 20);
    }

    #[test]
    fn test_vec3_scalar_sum_with_zero_vec() {
        let a: Vec3 = Vec3Ops::default_vec3();
        let b = [4, 4, 6];
        let c = a.vec3_scalar_sum(b);
        assert_eq!(c, 14);
    }
}
