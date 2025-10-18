use bytemuck::{Pod, Zeroable};

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::common;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random_in_range(min: f32, max: f32) -> Self {
        Vec3 {
            x: common::random_in_range(min, max),
            y: common::random_in_range(min, max),
            z: common::random_in_range(min, max),
        }
    }


    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        const EPS: f32 = 1.0e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, v: Vec3) -> Self {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        *self = *self - v;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, v: Vec3) -> Self {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        *self = *self * v
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, c: f32) {
        *self = *self * c
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Vec3) -> Self {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, c: f32) -> Self {
        Vec3::new(self.x() * c, self.y() * c, self.z() * c)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.x() * self, v.y() * self, v.z() * self)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        *self = *self / v
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, d: f32) {
        *self = *self / d
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, v: Vec3) -> Self {
        Vec3::new(self.x() / v.x(), self.y() / v.y(), self.z() / v.z())
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, d: f32) -> Self {
        Vec3::new(self.x() / d, self.y() / d, self.z() / d)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * dot(v, normal) * normal
}

pub fn refract(v: Vec3, normal: Vec3, refractive_ratio: f32) -> Vec3 {
    let cos_theta = f32::min(dot(-v, normal), 1.0);
    let perp = refractive_ratio * (v + cos_theta * normal);
    let parallel = -f32::sqrt(f32::abs(1.0 - perp.length_squared())) * normal;
    perp + parallel
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::random_in_range(-1.0, 1.0);
        if v.length_squared() >= 1.0 {
            continue;
        }

        return v;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}
