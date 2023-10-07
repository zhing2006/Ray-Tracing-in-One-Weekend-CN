use std::rc::Rc;

use super::vec3::{self, Vec3, Point3};
use super::ray::Ray;
use super::interval::Interval;
use super::material::Material;

#[derive(Clone, Default)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub mat: Option<Rc<dyn Material>>,
  pub t: f64,
  pub front_face: bool,
}

pub trait Hittable {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    // Sets the hit record normal vector.
    // NOTE: the parameter `outward_normal` is assumed to have unit length.

    self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      -*outward_normal
    };
  }
}