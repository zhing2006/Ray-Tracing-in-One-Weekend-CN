use super::vec3::Point3;
use super::interval::{
  self,
  Interval,
};
use super::ray::Ray;

#[derive(Default, Clone)]  // 默认的AABB是空的，因为区间默认是空的。
pub struct Aabb {
  pub x: Interval,
  pub y: Interval,
  pub z: Interval,
}

impl Aabb {
  pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
    Self {
      x: (*x).clone(),
      y: (*y).clone(),
      z: (*z).clone(),
    }
  }

  pub fn new_with_point(a: &Point3, b: &Point3) -> Self {
    // 将两个点a和b视为包围盒的极值，这样我们不需要特定的最小/最大坐标顺序。
    let mut _self = Self {
      x: Interval::new((a[0]).min(b[0]), (a[0]).max(b[0])),
      y: Interval::new((a[1]).min(b[1]), (a[1]).max(b[1])),
      z: Interval::new((a[2]).min(b[2]), (a[2]).max(b[2])),
    };
    _self.pad_to_minimums();
    _self
  }

  pub fn new_with_box(box0: &Aabb, box1: &Aabb) -> Self {
    Self {
      x: Interval::new_with_interval(&box0.x, &box1.x),
      y: Interval::new_with_interval(&box0.y, &box1.y),
      z: Interval::new_with_interval(&box0.z, &box1.z),
    }
  }

  pub fn axis(&self, n: usize) -> &Interval {
    match n {
      0 => &self.x,
      1 => &self.y,
      _ => &self.z,
    }
  }

  pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
    for a in 0..3 {
      let inv0 = 1.0 / r.direction()[a];
      let orig = r.origin()[a];

      let mut t0 = (self.axis(a).min - orig) * inv0;
      let mut t1 = (self.axis(a).max - orig) * inv0;

      if inv0 < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
      }

      if t0 > ray_t.min {
        ray_t.min = t0;
      }
      if t1 < ray_t.max {
        ray_t.max = t1;
      }

      if ray_t.max <= ray_t.min {
        return false;
      }
    }
    true
  }

  pub fn longest_axis(&self) -> usize {
    // 返回边界框的最长轴的索引。
    if self.x.size() > self.y.size() {
      if self.x.size() > self.z.size() {
        0
      } else {
        2
      }
    } else if self.y.size() > self.z.size() {
      1
    } else {
      2
    }
  }

  fn pad_to_minimums(&mut self) {
    // 调整AABB，使得没有一边比某个delta更窄，如果需要的话进行填充。
    let delta = 0.0001;
    if self.x.size() < delta {
      self.x.expand(delta);
    }
    if self.y.size() < delta {
      self.y.expand(delta);
    }
    if self.z.size() < delta {
      self.z.expand(delta);
    }
  }
}

pub const EMPTY: Aabb = Aabb {
  x: interval::EMPTY,
  y: interval::EMPTY,
  z: interval::EMPTY,
};
pub const UNIVERSE: Aabb = Aabb {
  x: interval::UNIVERSE,
  y: interval::UNIVERSE,
  z: interval::UNIVERSE,
};