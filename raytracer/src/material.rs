use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    refractive_index: f32,
    albedo: [f32; 4],
    diffuse_color: Vec3,
    specular_exponent: f32,
}

impl Material {
    pub fn new(
        refractive_index: f32,
        albedo: [f32; 4],
        diffuse_color: Vec3,
        specular_exponent: f32,
    ) -> Self {
        Self {
            refractive_index,
            albedo,
            diffuse_color,
            specular_exponent,
        }
    }

    pub fn refractive_index(&self) -> f32 {
        self.refractive_index
    }

    pub fn albedo(&self) -> &[f32; 4] {
        &self.albedo
    }

    pub fn diffuse_color(&self) -> Vec3 {
        self.diffuse_color
    }

    pub fn specular_exponent(&self) -> f32 {
        self.specular_exponent
    }

    pub fn set_refractive_index(&mut self, refractive_index: f32) {
        self.refractive_index = refractive_index;
    }

    pub fn set_albedo(&mut self, albedo: [f32; 4]) {
        self.albedo = albedo;
    }

    pub fn set_diffuse_color(&mut self, diffuse_color: Vec3) {
        self.diffuse_color = diffuse_color;
    }

    pub fn set_specular_exponent(&mut self, specular_exponent: f32) {
        self.specular_exponent = specular_exponent;
    }
}

impl Default for Material {
    fn default() -> Material {
        Material {
            refractive_index: 1.0,
            albedo: [2.0, 0.0, 0.0, 0.0],
            diffuse_color: Vec3::default(),
            specular_exponent: 0.0,
        }
    }
}
