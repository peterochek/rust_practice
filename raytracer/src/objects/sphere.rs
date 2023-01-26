use crate::material::Material;
use crate::objects::object::Object;
use crate::raytracing::util::EPS;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Object for Sphere {
    fn intersect(&self, orig: Vec3, dir: Vec3) -> (bool, f32) {
        let l = self.center - orig;
        let tca = l * dir;
        let d2 = l * l - tca * tca;
        let r2 = self.radius.powi(2);
        if d2 > r2 {
            return (false, 0.0);
        }
        let thc = (r2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 > EPS {
            return (true, t0);
        }
        if t1 > EPS {
            return (true, t1);
        }
        (false, 0.0)
    }

    fn center(&self) -> Vec3 {
        self.center
    }

    fn material(&self, _p: Vec3) -> Material {
        self.material
    }

    fn norm(&self, p: Vec3) -> Vec3 {
        (p - self.center()).norm()
    }
}
