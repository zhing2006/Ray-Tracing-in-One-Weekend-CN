use the_rest_of_your_life::rtweekend::random_double;

fn f(d: f64) -> f64 {
  8.0 * d.powf(1.0 / 3.0)
}

fn pdf(x: f64) -> f64 {
  3.0 / 8.0 * x * x
}

fn main() {
  const N: usize = 1;
  let mut sum = 0.0;
  (0..N).for_each(|_| {
    let x = f(random_double());
    sum += x * x / pdf(x);
  });
  println!("I = {:.12}", sum / N as f64);
}