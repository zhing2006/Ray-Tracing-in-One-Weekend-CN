# 使用Rust实现RayTracingTheNextWeek

## 初始化

使用`cargo`创建工程

    cargo new --name the_next_week TheNextWeek



## 运动模糊

### SpaceTime光线追踪简介

```rust
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
+   tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir,
+           tm: 0.0,
        }
    }

+   pub fn new_with_time(orig: Point3, dir: Vec3, tm: f64) -> Self {
+       Self {
+           orig,
+           dir,
+           tm,
+       }
+   }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

+   pub fn time(&self) -> f64 {
+       self.tm
+   }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
```
Listing 1: [ray.rs] 带有时间信息的光线


### 更新相机以模拟运动模糊

```rust
impl Camera {
      fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
+       let ray_time = rtweekend::random_double();

+       Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }
}
```
Listing 2: [camera.rs] 带有时间信息的相机

```rust
pub struct Sphere {
+   center1: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
+   is_moving: bool,
+   center_vec: Vec3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
+           center1: center,
            radius,
            mat: material,
+           is_moving: false,
+           center_vec: Vec3::default(),
        }
    }

+   pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       Self {
+           center1,
+           radius,
+           mat: material,
+           is_moving: true,
+           center_vec: center2 - center1,
+       }
+   }
+
+   fn sphere_center(&self, time: f64) -> Point3 {
+       self.center1 + self.center_vec * time
+   }

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        ...

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
+       let outward_normal = (hit_record.p - self.center1) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
}
```
Listing 3: [sphere.rs] 移动的球体

```rust
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
+       let center = if self.is_moving { self.sphere_center(r.time()) } else { self.center1 };
+       let oc = center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        ...
    }
}
```
Listing 4: [sphere.rs] 移动的球体的碰撞函数


### 追踪光线交点的时间

```rust
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // 捕捉退化的散射方向
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

+       *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
+       *scattered = Ray::new_with_time(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo;
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        ...
+       *scattered = Ray::new_with_time(rec.p, direction, r_in.time());
        true
    }
}
```
Listing 5: [material.rs] 在 material::scatter() 方法中处理光线时间


### 将所有内容放在一起

```rust
fn main() {
    ...

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };

+               let center2 = center + vec3::Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
                world.add(Rc::new(
+                   Sphere::new_with_center2(center, center2, 0.2, sphere_material)
                ));
            }
        }
    }

    ...
}
```
Listing 6: [main.rs] 上一本书的最终场景，但是球会移动

![图像 1: 弹跳的球](../../images/img-2.01-bouncing-spheres.png)


## 包围体层次结构

### 光线与AABB的相交

```rust
+#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

+impl Default for Interval {
+   fn default() -> Self {
+       Self {
+           min: rtweekend::INFINITY,
+           max: -rtweekend::INFINITY,
+       }
+   }
+}

impl Interval {
    ...
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

+   pub fn expand(&mut self, delta: f64) -> Interval {
+       let padding = delta / 2.0;
+       Interval::new(self.min - padding, self.max + padding)
+   }

    ...
}
```
Listing 7: [interval.rs] interval::expand() 方法

```rust
use super::vec3::Point3;
use super::interval::Interval;
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

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let t0 = ((self.axis(a).min - r.origin()[a]) / r.direction()[a]).min(
                (self.axis(a).max - r.origin()[a]) / r.direction()[a],
            );
            let t1 = ((self.axis(a).min - r.origin()[a]) / r.direction()[a]).max(
                (self.axis(a).max - r.origin()[a]) / r.direction()[a],
            );
            ray_t.min = t0.max(ray_t.min);
            ray_t.max = t1.min(ray_t.max);
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
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
```
Listing 8: [aabb.rs] 轴对齐包围盒类


### 优化的AABB相交方法

```rust
impl Aabb {
    ...
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
    ...
}
```
Listing 9: [aabb.rs] 可选的优化的AABB相交函数


### 构建可击中物体的包围盒

```rust
pub trait Hittable {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
  fn bounding_box(&self) -> &Aabb;
}
```
Listing 10: [hittable.rs] 具有包围盒的可击中物体类

```rust
pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
+   bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       let rvec = Vec3::new(radius, radius, radius);
        Self {
            center1: center,
            radius,
            mat: material,
            is_moving: false,
            center_vec: Vec3::default(),
+           bbox: Aabb::new_with_point(&(center - rvec), &(center + rvec)),
        }
    }
    ...
}

impl Hittable for Sphere {
    ...

+   pub fn bounding_box(&self) -> &Aabb {
+       &self.bbox
+   }
    ...
}
```
Listing 11: [sphere.rs] 具有包围盒的球体类

```rust
impl Sphere {
    ...
    pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       let rvec = Vec3::new(radius, radius, radius);
+       let box1 = Aabb::new_with_point(&(center1 - rvec), &(center1 + rvec));
+       let box2 = Aabb::new_with_point(&(center2 - rvec), &(center2 + rvec));
        Self {
            center1,
            radius,
            mat: material,
            is_moving: true,
            center_vec: center2 - center1,
+           bbox: Aabb::new_with_box(&box1, &box2),
        }
    }
    ...
}
```
Listing 12: [sphere.rs] 具有包围盒的移动球体类

```rust
impl Interval {
    ...
+   pub fn new_with_interval(a: &Interval, b: &Interval) -> Self {
+       Self {
+           min: a.min.min(b.min),
+           max: a.max.max(b.max),
+       }
+   }
    ...
}
```
Listing 13: [interval.rs] 从两个区间构造区间的构造函数

```rust
impl Aabb {
    ...
+   pub fn new_with_box(box0: &Aabb, box1: &Aabb) -> Self {
+       Self {
+           x: Interval::new_with_interval(&box0.x, &box1.x),
+           y: Interval::new_with_interval(&box0.y, &box1.y),
+           z: Interval::new_with_interval(&box0.z, &box1.z),
+       }
+   }
    ...
}
```
Listing 14: [aabb.rs] 从两个AABB输入构造AABB


### 创建物体列表的包围盒

```rust
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
+   bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
+           bbox: Aabb::default(),
        }
    }

    ...

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
+       self.bbox = Aabb::new_with_box(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    ...

+   fn bounding_box(&self) -> &Aabb {
+       &self.bbox
+   }
}
```
Listing 15: [hittable_list.rs] 具有包围盒的可击中物体列表


### BVH节点类

```rust
use std::rc::Rc;

use super::hittable::{
    Hittable,
    HitRecord,
};
use super::hittable_list::HittableList;
use super::ray::Ray;
use super::interval::Interval;
use super::aabb::Aabb;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList) -> Self {
        let len = list.objects.len();
        Self::new_with_hitables(&mut list.objects, 0, len)
    }

    pub fn new_with_hitables(src_objects: &Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // TODO
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
        &elf.bbox
    }
}
```
Listing 16: [bvh.rs] 包围体层次结构


### 分割BVH体积

```rust
impl BvhNode {
    ...
    pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = rtweekend::random_int(0, 2);

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
                bbox: (*objects[start].bounding_box()).clone(),
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bbox: Aabb::new_with_box(
                        objects[start].bounding_box(),
                        objects[start + 1].bounding_box(),
                    ),
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bbox: Aabb::new_with_box(
                        objects[start + 1].bounding_box(),
                        objects[start].bounding_box(),
                    ),
                }
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Rc::new(Self::new_with_hitables(objects, start, mid));
            let right = Rc::new(Self::new_with_hitables(objects, mid, end));
            let bbox = Aabb::new_with_box(left.bounding_box(), right.bounding_box());
            Self {
                left,
                right,
                bbox,
            }
        }
    }
    ...
}
```
Listing 17: [bvh.rs] 包围体层次结构节点

```rust
pub fn random_int(min: i32, max: i32) -> i32 {
    // Returns a random integer in [min,max].
    random_double_range(min as f64, (max + 1) as f64) as i32
}
```
Listing 18: [rtweekend.rs] 返回指定范围内的随机整数的函数


### 盒子比较函数

```rust
impl BvhNode {
    ...
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
```
Listing 19: [bvh.rs] BVH 比较函数，X 轴

```rust
fn main() {
    ...

+   let world = HittableList::new(Rc::new(BvhNode::new(&mut world)));

    // Camera
    ...
}
```
Listing 20: [main.rs] 随机球体，使用 BVH


### 另一个 BVH 优化

```rust
impl BvhNode {
  ...

  pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
+   // 构建源对象范围的边界框。
+   let mut bbox = Aabb::default();
+   src_objects[start..end].iter().for_each(|obj| {
+       bbox = Aabb::new_with_box(&bbox, obj.bounding_box());
+   });

    ...
```
Listing 21: [bvh.rs] 构建 BVH 对象范围的边界框

```rust
    pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
+       let axis = bbox.longest_axis();
        ...
        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
+               bbox,
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
+               bbox,
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
+                   bbox,
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
+               bbox,
            }
        }
    }
```
Listing 22: [bvh.rs] 构建 BVH 对象范围的边界框

```rust
impl Aabb {
    ...

+   pub fn longest_axis(&self) -> usize {
+       // 返回边界框的最长轴的索引。
+       if self.x.size() > self.y.size() {
+           if self.x.size() > self.z.size() {
+               0
+           } else {
+               2
+           }
+       } else if self.y.size() > self.z.size() {
+           1
+       } else {
+           2
+       }
+   }

    ...
}

+pub const EMPTY: Aabb = Aabb {
+   x: interval::EMPTY,
+   y: interval::EMPTY,
+   z: interval::EMPTY,
+};
+pub const UNIVERSE: Aabb = Aabb {
+   x: interval::UNIVERSE,
+   y: interval::UNIVERSE,
+   z: interval::UNIVERSE,
+};
```
Listing 23: [aabb.rs] 新的 aabb 常量和 longest_axis() 函数



## 纹理映射

### 常量颜色纹理

```rust
use super::vec3::Point3;
use super::color::Color;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self {
            color_value,
        }
    }

    pub fn new_with_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}
```
Listing 24: [texture.rs] 纹理类

```rust
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
+   pub u: f64,
+   pub v: f64,
    pub front_face: bool,
}
```
Listing 25: [hittable.rs] 将 u,v 坐标添加到 hit_record


### 实体纹理：棋盘格纹理

```rust
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_with_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColor::new(c1)),
            odd: Rc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
```
Listing 26: [texture.rs] 棋盘格纹理

```rust
pub struct Lambertian {
+   pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
+           albedo: Rc::new(SolidColor::new(a)),
        }
    }

+   pub fn new_with_texture(a: Rc<dyn Texture>) -> Self {
+       Self {
+           albedo: a,
+       }
+   }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // 捕捉退化的散射方向
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
+        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
```
Listing 27: [material.rs] 带有纹理的朗伯材质

```rust
fn main() {
    ...
+   let checker: Rc<dyn Texture> = Rc::new(
+       CheckerTexture::new_with_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9))
+   );
    let ground_material: Rc<dyn Material> = Rc::new(
+       Lambertian::new_with_texture(Rc::clone(&checker))
    );
    world.add(Rc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));
    ...
}
```
Listing 28: [main.rs] 使用棋盘格纹理

![图像 2：棋盘格地面上的球体](../../images/img-2.02-checker-ground.png)


### 渲染实体棋盘格纹理

```rust
fn random_spheres() {
    ...
    let ground_material: Rc<dyn Material> = Rc::new(
        Lambertian::new(color::Color::new(0.5, 0.5, 0.5))
    );
    world.add(Rc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));
    ...
}

fn main() {
    random_spheres();
}
```
Listing 29: [main.rs] 主函数调用选定的场景

```rust
fn two_spheres() {
    let mut world = HittableList::default();

    let checker: Rc<dyn Texture> = Rc::new(CheckerTexture::new_with_color(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Rc::new(
        Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            Rc::new(Lambertian::new_with_texture(Rc::clone(&checker)))
        )
    ));
    world.add(Rc::new(
        Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            Rc::new(Lambertian::new_with_texture(Rc::clone(&checker)))
        )
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 2 {
        1 => random_spheres(),
        2 => two_spheres(),
        _ => (),
    }
}
```
Listing 30: [main.rs] 两个带纹理的球体

![图像 3：棋盘格球体](../../images/img-2.03-checker-spheres.png)


### 球体的纹理坐标

```rust
impl Sphere {
    ...

+    fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
+       // p: a given point on the sphere of radius one, centered at the origin.
+       // u: returned value [0,1] of angle around the Y axis from X=-1.
+       // v: returned value [0,1] of angle from Y=-1 to Y=+1.
+       //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
+       //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
+       //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
+
+       let theta = (-p.y()).acos();
+       let phi = (-p.z()).atan2(p.x()) + rtweekend::PI;
+
+       *u = phi / (2.0 * rtweekend::PI);
+       *v = theta / rtweekend::PI;
+   }
}
```
Listing 31: [sphere.rs] get_sphere_uv 函数

```rust
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        ...
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center1) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
+       Self::get_sphere_uv(outward_normal, &mut hit_record.u, &mut hit_record.v);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
    ...
}
```
Listing 32: [sphere.rs] 从命中点获取球体的UV坐标


### 访问纹理图像数据
