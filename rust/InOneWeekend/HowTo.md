# 使用Rust实现RayTracingInOneWeekend

## 初始化

使用`cargo`创建工程

    cargo new --name in_one_weekend InOneWeekend

## 输出图像

### PPM图像格式

修改main.rs为：

```rust
fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
```
[main.rs]创建你的第一张图像

### 创建图像文件

构建：

    cargo build

运行：

    target/debug/in_one_weekend > image.ppm

构建Release：

    cargo build --release

运行Release：

    target/release/in_one_weekend > image.ppm

![Image 1：第一个PPM图像](../../images/img-1.01-first-ppm-image.png)

### 添加进度指示器

```rust
fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
+       eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
+
+   eprintln!("\nDone.");
}
```
_[main.rs] 主渲染循环与进度报告_

## vec3类

```rust
use std::ops::{
    AddAssign,
    DivAssign,
    Index,
    IndexMut,
    MulAssign,
    Neg,
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        (1 as f64 / t) * self
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

pub type Point3 = Vec3;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 { e: [
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    ]}
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    (*v).clone() / v.length()
}
```
_[vec3.rs]Vec3 定义和辅助函数_

需要注意的是和C++不同Rust不能重载运算符，需要通过实现不同运算符的Trait来实现相关功能。另外这里通过实现std::fmt::Display这个Trait来达到C++中ostream输出的功能。还有由于Rust的所有权特性，这里为Vec3实现了Copy Trait，这样更接近C++中vec3的行为。

### 颜色实用函数

```rust
use std::io::Write;

use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write) -> std::io::Result<()> {
        write!(out, "{} {} {}\n",
        (255.999 * self.x()) as i32,
        (255.999 * self.y()) as i32,
        (255.999 * self.z()) as i32)
    }
}
```
_[color.rs]Color 实用函数_

然后修改main.rs：

```rust
+pub mod vec3;
+pub mod color;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
+   let stdout = std::io::stdout();

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
+           let pixel_color = color::Color::new(
+               i as f64 / (image_width - 1) as f64,
+               j as f64 / (image_height - 1) as f64,
+               0.0);
+           pixel_color.write_color(&mut stdout.lock()).unwrap();
        }
    }

    eprintln!("\nDone.");
}
```
_[main.rs] 第一个PPM图像的最终代码_

## 光线、简单相机和背景

### 光线类

```rust
use super::vec3::{
    Vec3,
    Point3,
};

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
        orig: Vec3::default(),
        dir: Vec3::default(),
        }
    }
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
        orig: *orig,
        dir: *dir,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
```
_[ray.rs] 光线类_

### 发送光线到场景中

```rust
let aspect_ratio = 16.0 / 9.0;
let image_width = 400;

// 计算图像高度，并确保至少为1。
let image_height = image_width / aspect_ratio as i32;
let image_height = if image_height < 1 { 1 } else { image_height };

// 视口宽度小于1是可以的，因为它们是实值。
let viewport_height = 2.0;
let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
```
_渲染设置_

```rust
pub mod vec3;
+pub mod color;
pub mod ray;

use vec3::{
    Vec3,
    Point3,
};
use color::Color;
use ray::Ray;

+fn ray_color(r: &Ray) -> Color {
+   Color::new(0.0, 0.0, 0.0)
}

fn main() {
    // Image
+   let aspect_ratio = 16.0 / 9.0;
+   let image_width = 400;
+
+   // 计算图像高度，并确保至少为1。
+   let image_height = image_width / aspect_ratio as i32;
+   let image_height = if image_height < 1 { 1 } else { image_height };
+
+   // Camera
+   let focal_length = 1.0;
+   let viewport_height = 2.0;
+   let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
+   let camera_center = Point3::default();
+
+   // 计算水平和垂直视口边缘上的向量。
+   let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
+   let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
+
+   // 计算从像素到像素的水平和垂直增量向量。
+   let pixel_delta_u = viewport_u / image_width as f64;
+   let pixel_delta_v = viewport_v / image_height as f64;
+
+   // 计算左上角像素的位置。
+   let viewport_upper_left = camera_center
+       - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
+   let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    let stdout = std::io::stdout();

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
+           let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
+           let ray_direction = pixel_center - camera_center;
+           let r = Ray::new(&camera_center, &ray_direction);
+
+           let pixel_color = ray_color(&r);
            pixel_color.write_color(&mut stdout.lock()).unwrap();
        }
    }

    eprintln!("\nDone.");
}
```
_[main.rs] 创建场景光线_

```rust
pub mod vec3;
pub mod color;
pub mod ray;

use vec3::{
    Vec3,
    Point3,
};
use color::Color;
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
+   let unit_direction = vec3::unit_vector(r.direction());
+   let a = 0.5 * (unit_direction.y() + 1.0);
+   (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

...
```
_[main.rs] 渲染一个蓝白渐变_

![Image 2: 根据射线的Y坐标产生蓝白渐变](../../images/img-1.02-blue-to-white.png)