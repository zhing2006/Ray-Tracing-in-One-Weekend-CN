pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

pub fn random_double() -> f64 {
  // Returns a random real in [0,1).
  rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
  // Returns a random real in [min,max).
  min + (max - min) * random_double()
}

pub fn random_int(min: i32, max: i32) -> i32 {
  // Returns a random integer in [min,max].
  random_double_range(min as f64, (max + 1) as f64) as i32
}