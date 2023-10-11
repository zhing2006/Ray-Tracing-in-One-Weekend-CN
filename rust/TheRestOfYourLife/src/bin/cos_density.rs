use the_rest_of_your_life::rtweekend;
use the_rest_of_your_life::vec3::{self, Vec3};

fn f(d: Vec3) -> f64 {
  let cos_theta = d.z();
  cos_theta * cos_theta * cos_theta
}

fn pdf(d: Vec3) -> f64 {
  d.z() / rtweekend::PI
}

fn main() {
  const N: usize = 1000000;

  let mut sum = 0.0;
  (0..N).for_each(|_| {
    let d = vec3::random_cosine_direction();
    sum += f(d) / pdf(d);
  });

  println!("PI/2 = {:.12}", rtweekend::PI / 2.0);
  println!("Estimate = {:.12}", sum / N as f64);
}