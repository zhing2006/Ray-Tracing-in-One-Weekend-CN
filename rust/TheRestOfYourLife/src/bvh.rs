use std::rc::Rc;

use super::hittable::{
  Hittable,
  HitRecord,
};
use super::hittable_list::HittableList;
use super::ray::Ray;
use super::interval::Interval;
use super::aabb::{self, Aabb};

pub struct BvhNode {
  left: Rc<dyn Hittable>,
  right: Rc<dyn Hittable>,
  bbox: Aabb,
}

impl BvhNode {
  pub fn new(list: &mut HittableList) -> Self {
    let len = list.objects.len();
    Self::new_with_hitables(&mut list.objects, 0, len)
  }

  pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
    // 构建源对象范围的边界框。
    let mut bbox = aabb::EMPTY;
    src_objects[start..end].iter().for_each(|obj| {
      bbox = Aabb::new_with_box(&bbox, obj.bounding_box());
    });

    let axis = bbox.longest_axis();

    let comparator = match axis {
      0 => Self::box_x_compare,
      1 => Self::box_y_compare,
      _ => Self::box_z_compare,
    };

    let objects = src_objects;

    let object_span = end - start;

    if object_span == 1 {
      Self {
        left: objects[start].clone(),
        right: objects[start].clone(),
        bbox,
      }
    } else if object_span == 2 {
      if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
        Self {
          left: objects[start].clone(),
          right: objects[start + 1].clone(),
          bbox,
        }
      } else {
        Self {
          left: objects[start + 1].clone(),
          right: objects[start].clone(),
          bbox,
        }
      }
    } else {
      objects[start..end].sort_by(comparator);

      let mid = start + object_span / 2;
      let left = Rc::new(Self::new_with_hitables(objects, start, mid));
      let right = Rc::new(Self::new_with_hitables(objects, mid, end));
      Self {
        left,
        right,
        bbox,
      }
    }
  }

  fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: usize) -> std::cmp::Ordering {
    a.bounding_box().axis(axis_index).min.partial_cmp(&b.bounding_box().axis(axis_index).min).unwrap()
  }

  fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    Self::box_compare(a, b, 0)
  }

  fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    Self::box_compare(a, b, 1)
  }

  fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    Self::box_compare(a, b, 2)
  }
}

impl Hittable for BvhNode {
  fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    let mut ray_t = ray_t.clone();
    if !self.bbox.hit(r, &mut ray_t) {
      return false;
    }

    let hit_left = self.left.hit(r, &ray_t, rec);
    let ray_t = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
    let hit_right = self.right.hit(r, &ray_t, rec);

    hit_left || hit_right
  }

  fn bounding_box(&self) -> &Aabb {
    &self.bbox
  }
}
