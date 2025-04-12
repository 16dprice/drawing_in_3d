use macroquad::math::{Vec3, vec3};

// R is radius from center of torus to center of tube
// r is radius of tube
pub struct Torus {
    pub R: f32,
    pub r: f32,
}

impl Torus {
    pub fn get_point(&self, theta: f32, phi: f32) -> Vec3 {
        return vec3(
            (self.R + self.r * f32::sin(theta)) * f32::cos(phi),
            (self.R + self.r * f32::sin(theta)) * f32::sin(phi),
            self.r * f32::cos(theta),
        )
    }
}
