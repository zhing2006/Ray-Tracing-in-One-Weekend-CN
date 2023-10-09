use std::rc::Rc;

use super::hittable::{
  HitRecord,
  Hittable,
};
use super::ray::Ray;
use super::interval::Interval;
use super::aabb::Aabb;

#[derive(Default)]
pub struct HittableList {
  pub objects: Vec<Rc<dyn Hittable>>,
  bbox: Aabb,
}

impl HittableList {
  pub fn new(object: Rc<dyn Hittable>) -> Self {
    Self {
      objects: vec![object],
      bbox: Aabb::default(),
    }
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, object: Rc<dyn Hittable>) {
    self.bbox = Aabb::new_with_box(&self.bbox, object.bounding_box());
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_so_far = ray_t.max;

    for object in self.objects.iter() {
      if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec.clone();
      }
    }

    hit_anything
  }

  fn bounding_box(&self) -> &Aabb {
    &self.bbox
  }
}