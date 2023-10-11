use the_rest_of_your_life::rtweekend::random_double_range;

fn main() {
  const N: usize = 100000;
  let mut inside_circle = 0;
  (0..N).for_each(|_| {
    let x = random_double_range(-1.0, 1.0);
    let y = random_double_range(-1.0, 1.0);
    if x * x + y * y < 1.0 {
      inside_circle += 1;
    }
  });
  println!("Estimated area of unit circle: {:.12}", 4.0 * inside_circle as f64 / N as f64);
}