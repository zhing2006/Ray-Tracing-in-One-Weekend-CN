use super::rtweekend;

#[derive(Default)]
pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub fn new(min: f64, max: f64) -> Self {
    Self { min, max }
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }
}

pub const EMPTY: Interval = Interval {
  min: rtweekend::INFINITY,
  max: -rtweekend::INFINITY,
};
pub const UNIVERSE: Interval = Interval {
  min: -rtweekend::INFINITY,
  max: rtweekend::INFINITY,
};