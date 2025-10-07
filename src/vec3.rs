use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::common;

#[derive(Copy, Clone, Default)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Vec3(
            common::random_in_range(min, max),
            common::random_in_range(min, max),
            common::random_in_range(min, max),
        )
    }


    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
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

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, c: f64) {
        *self = *self * c
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Vec3) -> Self {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, c: f64) -> Self {
        Vec3::new(self.x() * c, self.y() * c, self.z() * c)
    }
}

impl Mul<Vec3> for f64 {
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

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, d: f64) {
        *self = *self / d
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, v: Vec3) -> Self {
        Vec3::new(self.x() / v.x(), self.y() / v.y(), self.z() / v.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, d: f64) -> Self {
        Vec3::new(self.x() / d, self.y() / d, self.z() / d)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
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
