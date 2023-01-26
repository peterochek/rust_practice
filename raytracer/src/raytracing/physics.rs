use crate::material::Material;
use crate::objects::object::Object;
use crate::raytracing::util::{CLOSEST_VIEW_DISTANCE, MAX_REFLECTION_DEPTH};
use crate::vec3::Vec3;
use std::sync::Arc;

fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - n * 2.0 * (i * n)
}

fn refract(i: Vec3, n: Vec3, eta_t: f32, eta_i: f32) -> Vec3 {
    let cos = -(f32::max(-1.0, f32::min(1.0, i * n)));
    if cos < 0.0 {
        return refract(i, -n, eta_i, eta_t);
    }
    let eta = eta_i / eta_t;
    let k = 1.0 - eta.powi(2) * (1.0 - cos.powi(2));
    if k.is_sign_negative() {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        i * eta + n * (eta * cos - k.sqrt())
    }
}

fn scene_intersect(
    orig: Vec3,
    dir: Vec3,
    objects: &Vec<Arc<dyn Object + Sync + Send>>,
) -> (bool, Vec3, Vec3, Material) {
    let mut pt: Vec3 = Vec3::default();
    let mut n: Vec3 = Vec3::default();
    let mut material = Material::default();

    let mut nearest_dist = f32::MAX;

    for o in objects {
        let (intersection, d) = o.intersect(orig, dir);
        if !intersection || d > nearest_dist {
            continue;
        };
        nearest_dist = d;
        pt = orig + dir * nearest_dist;
        n = o.norm(pt);
        material = o.material(pt);
    }

    (nearest_dist < CLOSEST_VIEW_DISTANCE, pt, n, material)
}

pub fn cast_ray(
    orig: Vec3,
    dir: Vec3,
    lights: &Vec<Vec3>,
    objects: &Vec<Arc<dyn Object + Sync + Send>>,
    depth: i32,
) -> Vec3 {
    let (hit, point, n, material) = scene_intersect(orig, dir, objects);
    if depth > MAX_REFLECTION_DEPTH || !hit {
        return Vec3::new(0.2, 0.2, 0.2);
    }

    let reflect_dir = reflect(dir, n).norm();
    let refract_dir = refract(dir, n, material.refractive_index(), 1.0).norm();
    let reflect_color = cast_ray(point, reflect_dir, lights, objects, depth + 1);
    let refract_color = cast_ray(point, refract_dir, lights, objects, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;
    for light in lights {
        let light_dir = (*light - point).norm();
        let (hit, shadow_pt, _, _) = scene_intersect(point, light_dir, objects);
        if hit && (shadow_pt - point).length() < (*light - point).length() {
            continue;
        }
        diffuse_light_intensity += f32::max(0.0, light_dir * n);
        specular_light_intensity +=
            f32::max(0.0, -reflect(-light_dir, n) * dir).powf(material.specular_exponent());
    }

    material.diffuse_color() * diffuse_light_intensity * material.albedo()[0]
        + Vec3::new(1.0, 1.0, 1.0) * specular_light_intensity * material.albedo()[1]
        + reflect_color * material.albedo()[2]
        + refract_color * material.albedo()[3]
}
