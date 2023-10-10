use std::rc::Rc;

use super::vec3::{
  self,
  Vec3,
  Point3,
};
use super::ray::Ray;
use super::hittable::{
  HitRecord,
  Hittable,
};
use super::interval::Interval;
use super::material::Material;
use super::aabb::Aabb;
use super::rtweekend;

pub struct Sphere {
  center1: Point3,
  radius: f64,
  mat: Rc<dyn Material>,
  is_moving: bool,
  center_vec: Vec3,
  bbox: Aabb,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
    let rvec = Vec3::new(radius, radius, radius);
    Self {
      center1: center,
      radius,
      mat: material,
      is_moving: false,
      center_vec: Vec3::default(),
      bbox: Aabb::new_with_point(&(center - rvec), &(center + rvec)),
    }
  }

  pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
    let rvec = Vec3::new(radius, radius, radius);
    let box1 = Aabb::new_with_point(&(center1 - rvec), &(center1 + rvec));
    let box2 = Aabb::new_with_point(&(center2 - rvec), &(center2 + rvec));
    Self {
      center1,
      radius,
      mat: material,
      is_moving: true,
      center_vec: center2 - center1,
      bbox: Aabb::new_with_box(&box1, &box2),
    }
  }

  fn sphere_center(&self, time: f64) -> Point3 {
    self.center1 + self.center_vec * time
  }

  fn get_sphere_uv(p: Point3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + rtweekend::PI;

    (phi / (2.0 * rtweekend::PI), theta / rtweekend::PI)
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
    let center = if self.is_moving { self.sphere_center(r.time()) } else { self.center1 };
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = vec3::dot(r.direction(), oc);
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return false;
      }
    }

    hit_record.t = root;
    hit_record.p = r.at(hit_record.t);
    let outward_normal = (hit_record.p - self.center1) / self.radius;
    hit_record.set_face_normal(r, outward_normal);
    (hit_record.u, hit_record.v) = Self::get_sphere_uv(outward_normal);
    hit_record.mat = Some(Rc::clone(&self.mat));

    true
  }

  fn bounding_box(&self) -> &Aabb {
    &self.bbox
  }
}