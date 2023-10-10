use std::rc::Rc;

use super::rtweekend;
use super::vec3::{self, Vec3, Point3};
use super::ray::Ray;
use super::interval::Interval;
use super::material::Material;
use super::aabb::Aabb;

#[derive(Clone, Default)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub mat: Option<Rc<dyn Material>>,
  pub t: f64,
  pub u: f64,
  pub v: f64,
  pub front_face: bool,
}

pub trait Hittable {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
  fn bounding_box(&self) -> &Aabb;
}

impl HitRecord {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
    // Sets the hit record normal vector.
    // NOTE: the parameter `outward_normal` is assumed to have unit length.

    self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
    self.normal = if self.front_face {
      outward_normal
    } else {
      -outward_normal
    };
  }
}

pub struct Translate {
  object: Rc<dyn Hittable>,
  offset: Vec3,
  bbox: Aabb,
}

impl Translate {
  pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
    let bbox = object.bounding_box() + offset;
    Self {
      object,
      offset,
      bbox,
    }
  }
}

impl Hittable for Translate {
  fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    // 将光线向后移动偏移量
    let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());

    // 确定在偏移光线上是否存在交点（如果有，确定在哪里）
    if !self.object.hit(&offset_r, ray_t, rec) {
      return false;
    }

    // 将交点向前移动偏移量
    rec.p += self.offset;

    true
  }

  fn bounding_box(&self) -> &Aabb {
    &self.bbox
  }
}

pub struct RotateY {
  object: Rc<dyn Hittable>,
  sin_theta: f64,
  cos_theta: f64,
  bbox: Aabb,
}

impl RotateY {
  pub fn new(p: Rc<dyn Hittable>, angle: f64) -> Self {
    let radians = angle.to_radians();
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    let bbox = p.bounding_box();

    let mut min = Point3::new(rtweekend::INFINITY, rtweekend::INFINITY, rtweekend::INFINITY);
    let mut max = Point3::new(-rtweekend::INFINITY, -rtweekend::INFINITY, -rtweekend::INFINITY);

    (0..2).for_each(|i| {
      (0..2).for_each(|j| {
        (0..2).for_each(|k| {
          let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
          let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
          let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

          let newx = cos_theta * x + sin_theta * z;
          let newz = -sin_theta * x + cos_theta * z;

          let tester = Vec3::new(newx, y, newz);

          (0..3).for_each(|c| {
            min[c] = min[c].min(tester[c]);
            max[c] = max[c].max(tester[c]);
          })
        })
      })
    });

    let bbox = Aabb::new_with_point(&min, &max);
    Self {
      object: p,
      sin_theta,
      cos_theta,
      bbox,
    }
  }
}

impl Hittable for RotateY {
  fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    // 将光线从世界空间变换到对象空间
    let mut origin = r.origin();
    let mut direction = r.direction();

    origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
    origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

    direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
    direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

    let rotated_r = Ray::new_with_time(origin, direction, r.time());

    // 在对象空间中确定是否存在交点（如果有，确定在哪里）
    if !self.object.hit(&rotated_r, ray_t, rec) {
      return false;
    }

    // 将交点从对象空间变换到世界空间
    let mut p = rec.p;
    p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
    p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

    // 将法线从对象空间变换到世界空间
    let mut normal = rec.normal;
    normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
    normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

    rec.p = p;
    rec.normal = normal;

    true
  }

  fn bounding_box(&self) -> &Aabb {
    &self.bbox
  }
}