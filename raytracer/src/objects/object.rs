use crate::material::Material;
use crate::vec3::Vec3;

pub trait Object {
    fn intersect(&self, orig: Vec3, dir: Vec3) -> (bool, f32);

    fn center(&self) -> Vec3;

    fn material(&self, p: Vec3) -> Material;

    fn norm(&self, p: Vec3) -> Vec3;
}
