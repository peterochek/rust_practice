#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline]
    pub fn norm(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

use std::ops::{Add, Mul, Neg, Sub};

impl Add<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;
    #[inline]
    fn mul(self, rhs: Self) -> f32 {
        self.x.mul(rhs.x) + self.y.mul(rhs.y) + self.z.mul(rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn test_vec3_add_vec3() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(-4.0, 15.0, 7.0), vec1 + vec2);
    }

    #[test]
    fn test_vec3_add_f32() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let delta = 2.0;
        assert_eq!(Vec3::new(3.0, 7.0, 9.0), vec1 + delta);
    }

    #[test]
    fn test_vec3_sub_vec3() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(6.0, -5.0, 7.0), vec1 - vec2);
    }

    #[test]
    fn test_vec3_sub_f32() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let delta = 2.0;
        assert_eq!(Vec3::new(-1.0, 3.0, 5.0), vec1 - delta);
    }

    #[test]
    fn test_vec3_mul_vec3() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(45.0, vec1 * vec2);
    }

    #[test]
    fn test_vec3_mul_f32() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let delta = 2.0;
        assert_eq!(Vec3::new(2.0, 10.0, 14.0), vec1 * delta);
    }

    #[test]
    fn test_vec3_neg() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        assert_eq!(Vec3::new(-1.0, -5.0, -7.0), -vec1);
    }
}
