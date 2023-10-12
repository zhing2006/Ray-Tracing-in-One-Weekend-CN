use super::rtweekend;
use super::vec3::{self, Vec3, Point3};
use super::onb::Onb;
use super::hittable::Hittable;

pub trait Pdf {
  fn value(&self, direction: Vec3) -> f64;
  fn generate(&self) -> Vec3;
}

pub struct NonePdf;

impl Pdf for NonePdf {
  fn value(&self, _direction: Vec3) -> f64 {
    0.0
  }
  fn generate(&self) -> Vec3 {
    vec3::Vec3::new(1.0, 0.0, 0.0)
  }
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

pub struct HittablePdf<'a> {
  pub objects: &'a dyn Hittable,
  pub origin: Point3,
}

impl<'a> HittablePdf<'a> {
  pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
    Self {
      objects,
      origin,
    }
  }
}

impl Pdf for HittablePdf<'_> {
  fn value(&self, direction: Vec3) -> f64 {
    self.objects.pdf_value(self.origin, direction)
  }

  fn generate(&self) -> Vec3 {
    self.objects.random(self.origin)
  }
}

pub struct MixturePdf<'a> {
  pub p: [&'a dyn Pdf; 2],
}

impl<'a> MixturePdf<'a> {
  pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
    Self {
      p: [p0, p1],
    }
  }
}

impl Pdf for MixturePdf<'_> {
  fn value(&self, direction: Vec3) -> f64 {
    0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
  }

  fn generate(&self) -> Vec3 {
    if rtweekend::random_double_range(0.0, 1.0) < 0.5 {
      self.p[0].generate()
    } else {
      self.p[1].generate()
    }
  }
}