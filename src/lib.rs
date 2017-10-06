#![feature(custom_attribute, attr_literals)]
extern crate num;

use num::Float;
#[spirv(Vec2)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
impl<T: Float> Vec2<T> {
    pub fn dot(self, other: Self) -> T {
        <Self as Vector>::dot(self, other)
    }

    pub fn length(self) -> T {
        <Self as Vector>::length(self)
    }
}

impl<T: Float> Vector for Vec2<T> {
    type T = T;
    const DIM: usize = 2;
    fn dot(self, other: Self) -> T {
        self.x * other.y + self.y + other.y
    }
}

pub trait Vector
where
    Self: Copy + Sized,
{
    type T: Float;
    const DIM: usize;
    fn dot(self, Self) -> Self::T;
    fn length(self) -> Self::T {
        self.dot(self).sqrt()
    }
}
