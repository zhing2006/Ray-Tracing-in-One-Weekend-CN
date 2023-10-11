use std::rc::Rc;

use super::rtweekend;
use super::hittable::{
    Hittable,
    HitRecord,
};
use super::material::{
  Material,
  Isotropic,
};
use super::ray::Ray;
use super::vec3::Vec3;
use super::color::Color;
use super::aabb::Aabb;
use super::texture::Texture;
use super::interval::{
  self,
  Interval,
};

pub struct ConstantMedium {
  boundary: Rc<dyn Hittable>,
  neg_inv_density: f64,
  phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
  pub fn new(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
    Self {
      boundary: b,
      neg_inv_density: -1.0 / d,
      phase_function: Rc::new(Isotropic::new(a)),
    }
  }
  pub fn new_with_color(b: Rc<dyn Hittable>, d: f64, c: Color) -> Self {
    Self {
      boundary: b,
      neg_inv_density: -1.0 / d,
      phase_function: Rc::new(Isotropic::new_with_color(c)),
    }
  }
}

impl Hittable for ConstantMedium {
  fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    // Print occasional samples when debugging. To enable, set enableDebug true.
    const ENABLE_DEBUG: bool = false;
    let debugging = ENABLE_DEBUG && rtweekend::random_double() < 0.00001;

    let mut rec1 = HitRecord::default();
    let mut rec2 = HitRecord::default();

    if !self.boundary.hit(r, &interval::UNIVERSE, &mut rec1) {
      return false;
    }

    if !self.boundary.hit(r, &Interval::new(rec1.t + 0.0001, rtweekend::INFINITY), &mut rec2) {
      return false;
    }

    if debugging {
      eprintln!("\nray_tmin={} ray_tmax={}", rec1.t, rec2.t);
    }

    if rec1.t < ray_t.min {
      rec1.t = ray_t.min;
    }
    if rec2.t > ray_t.max {
      rec2.t = ray_t.max;
    }

    if rec1.t >= rec2.t {
      return false;
    }

    if rec1.t < 0.0 {
      rec1.t = 0.0;
    }

    let ray_length = r.direction().length();
    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
    let hit_distance = self.neg_inv_density * rtweekend::random_double().ln();

    if hit_distance > distance_inside_boundary {
      return false;
    }

    rec.t = rec1.t + hit_distance / ray_length;
    rec.p = r.at(rec.t);

    if debugging {
      eprintln!("hit_distance = {}", hit_distance);
      eprintln!("rec.t = {}", rec.t);
      eprintln!("rec.p = {}", rec.p);
    }

    rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
    rec.front_face = true; // also arbitrary
    rec.mat = Some(Rc::clone(&self.phase_function));

    true
  }

  fn bounding_box(&self) -> &Aabb {
    self.boundary.bounding_box()
  }
}