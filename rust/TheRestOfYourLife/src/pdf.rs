use super::rtweekend;
use super::vec3::{self, Vec3};
use super::onb::Onb;

pub trait Pdf {
  fn value(&self, direction: Vec3) -> f64;
  fn generate(&self) -> Vec3;
}

pub struct SpherePdf;

impl Pdf for SpherePdf {
  fn value(&self, _direction: Vec3) -> f64 {
    1.0 / (4.0 * rtweekend::PI)
  }

  fn generate(&self) -> Vec3 {
    vec3::random_unit_vector()
  }
}

pub struct CosinePdf {
  uvw: Onb,
}

impl CosinePdf {
  pub fn new(w: Vec3) -> Self {
    Self {
      uvw: Onb::new_from_w(w),
    }
  }
}

impl Pdf for CosinePdf {
  fn value(&self, direction: Vec3) -> f64 {
    let cosine_theta = vec3::dot(vec3::unit_vector(direction), self.uvw.w());
    0.0_f64.max(cosine_theta / rtweekend::PI)
  }

  fn generate(&self) -> Vec3 {
    self.uvw.local_v(vec3::random_cosine_direction())
  }
}