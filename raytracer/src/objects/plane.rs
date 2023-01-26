use crate::material::Material;
use crate::objects::object::Object;
use crate::raytracing::util::EPS;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Plane {
    center: Vec3,
    normal: Vec3,
    size: f32,
}

impl Plane {
    pub fn new(center: Vec3, normal: Vec3, size: f32) -> Self {
        Self {
            center,
            normal,
            size,
        }
    }
}

impl Object for Plane {
    fn intersect(&self, orig: Vec3, dir: Vec3) -> (bool, f32) {
        if (dir.y()).abs() > EPS {
            let d = -(orig.y() - self.center.y()) / dir.y();
            let p = orig + dir * d;
            let x_bounds = self.center.x() - self.size / 2.0 < p.x()
                && p.x() < self.center.x() + self.size / 2.0;
            let z_bounds = self.center.z() - self.size / 2.0 < p.z()
                && p.z() < self.center.z() + self.size / 2.0;
            if d > EPS && x_bounds && z_bounds {
                return (true, d);
            }
        }

        (false, 0.0)
    }

    fn center(&self) -> Vec3 {
        self.center
    }

    fn material(&self, p: Vec3) -> Material {
        let diffusive = if ((p.x() * p.z()) as i32) & 33 == 1 {
            Vec3::new(0.3, 0.1, 0.0)
        } else {
            Vec3::new(0.1, 0.1, 0.4)
        };

        Material::new(1.0, [2.0, 0.0, 0.0, 0.0], diffusive, 0.0)
    }

    fn norm(&self, _p: Vec3) -> Vec3 {
        self.normal
    }
}
