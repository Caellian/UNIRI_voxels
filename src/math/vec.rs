#![allow(unused_imports)]

use std::{cmp::Reverse, marker::PhantomData, ops::Index};

pub use bevy::math::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

use super::axis::{AsVecT, WorldAxis};
use super::mat::Mat3;

//use crate::convert::Convert;

enum IterDirUnit {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
}

impl From<IterDirUnit> for IVec3 {
    #[inline]
    fn from(value: IterDirUnit) -> Self {
        match value {
            IterDirUnit::X => IVec3::X,
            IterDirUnit::Y => IVec3::Y,
            IterDirUnit::Z => IVec3::Z,
            IterDirUnit::RX => IVec3::new(-1, 0, 0),
            IterDirUnit::RY => IVec3::new(0, -1, 0),
            IterDirUnit::RZ => IVec3::new(0, 0, -1),
        }
    }
}

impl From<IterDirUnit> for Vec3 {
    #[inline]
    fn from(value: IterDirUnit) -> Self {
        match value {
            IterDirUnit::X => Vec3::X,
            IterDirUnit::Y => Vec3::Y,
            IterDirUnit::Z => Vec3::Z,
            IterDirUnit::RX => Vec3::new(-1.0, 0.0, 0.0),
            IterDirUnit::RY => Vec3::new(0.0, -1.0, 0.0),
            IterDirUnit::RZ => Vec3::new(0.0, 0.0, -1.0),
        }
    }
}

pub struct X;
pub struct Y;
pub struct Z;

pub trait IterDirection {
    const UNIT: IterDirUnit;
    const AXIS: WorldAxis;
    const INVERSE: bool = false;

    #[inline(always)]
    fn component_of<V: IsVec>(v: V) -> V::Component
    where
        V: Index<WorldAxis, Output = V::Component>,
    {
        v[Self::AXIS]
    }

    #[inline(always)]
    fn zero<V>() -> V::Component
    where
        V: IsVec,
        V::Component: Copy,
    {
        V::ZERO[0]
    }
}
impl IterDirection for X {
    const UNIT: IterDirUnit = IterDirUnit::X;
    const AXIS: WorldAxis = WorldAxis::X;
}
impl IterDirection for Y {
    const UNIT: IterDirUnit = IterDirUnit::Y;
    const AXIS: WorldAxis = WorldAxis::Y;
}
impl IterDirection for Z {
    const UNIT: IterDirUnit = IterDirUnit::Z;
    const AXIS: WorldAxis = WorldAxis::Z;
}
impl IterDirection for Reverse<X> {
    const UNIT: IterDirUnit = IterDirUnit::RX;
    const AXIS: WorldAxis = WorldAxis::X;
    const INVERSE: bool = true;
}
impl IterDirection for Reverse<Y> {
    const UNIT: IterDirUnit = IterDirUnit::RY;
    const AXIS: WorldAxis = WorldAxis::Y;
    const INVERSE: bool = true;
}
impl IterDirection for Reverse<Z> {
    const UNIT: IterDirUnit = IterDirUnit::RZ;
    const AXIS: WorldAxis = WorldAxis::Z;
    const INVERSE: bool = true;
}

pub trait IsVec:
    Sized
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Self::Component, Output = Self>
    + std::ops::Div<Self::Component, Output = Self>
    + Index<usize, Output = Self::Component>
    + Clone
    + Copy
{
    const LENGTH: usize;

    const ZERO: Self;
    const UNIT: Self;

    type Component: Copy
        + PartialEq
        + PartialOrd
        + std::ops::Add<Output = Self::Component>
        + std::ops::Sub<Output = Self::Component>
        + std::ops::Mul<Output = Self::Component>
        + std::ops::Div<Output = Self::Component>
        + num::cast::NumCast;

    fn components(self) -> [Self::Component; Self::LENGTH];
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self;

    fn sum(self) -> Self::Component
    where
        [Self::Component; Self::LENGTH]: Sized,
    {
        unsafe {
            // SAFETY: No vector will ever have 0 components so checking for None is only a waste.
            self.components()
                .into_iter()
                .reduce(|acc, it| acc + it)
                .unwrap_unchecked()
        }
    }

    fn inner_product(self) -> Self::Component
    where
        [Self::Component; Self::LENGTH]: Sized,
    {
        unsafe {
            // SAFETY: No vector will ever have 0 components so checking for None is only a waste.
            self.components()
                .into_iter()
                .map(|it| it * it)
                .reduce(|acc, it| acc + it)
                .unwrap_unchecked()
        }
    }
}

impl IsVec for UVec4 {
    const LENGTH: usize = 4;
    const ZERO: Self = UVec4::ZERO;
    const UNIT: Self = UVec4::new(1, 1, 1, 1);

    type Component = u32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y, self.z, self.w]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for IVec4 {
    const LENGTH: usize = 4;
    const ZERO: Self = IVec4::ZERO;
    const UNIT: Self = IVec4::new(1, 1, 1, 1);

    type Component = i32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y, self.z, self.w]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for Vec4 {
    const LENGTH: usize = 4;
    const ZERO: Self = Vec4::ZERO;
    const UNIT: Self = Vec4::new(1.0, 1.0, 1.0, 1.0);

    type Component = f32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y, self.z, self.w]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for UVec3 {
    const LENGTH: usize = 3;
    const ZERO: Self = UVec3::ZERO;
    const UNIT: Self = UVec3::new(1, 1, 1);

    type Component = u32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y, self.z]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for IVec3 {
    const LENGTH: usize = 3;
    const ZERO: Self = IVec3::ZERO;
    const UNIT: Self = IVec3::new(1, 1, 1);

    type Component = i32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y, self.z]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for Vec3 {
    const LENGTH: usize = 3;
    const ZERO: Self = Vec3::ZERO;
    const UNIT: Self = Vec3::new(1.0, 1.0, 1.0);

    type Component = f32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y, self.z]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for UVec2 {
    const LENGTH: usize = 2;
    const ZERO: Self = UVec2::ZERO;
    const UNIT: Self = UVec2::new(1, 1);

    type Component = u32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for IVec2 {
    const LENGTH: usize = 2;
    const ZERO: Self = IVec2::ZERO;
    const UNIT: Self = IVec2::new(1, 1);

    type Component = i32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}
impl IsVec for Vec2 {
    const LENGTH: usize = 2;
    const ZERO: Self = Vec2::ZERO;
    const UNIT: Self = Vec2::new(1.0, 1.0);

    type Component = f32;

    #[inline(always)]
    fn components(self) -> [Self::Component; Self::LENGTH] {
        [self.x, self.y]
    }
    #[inline(always)]
    fn from_components(c: [Self::Component; Self::LENGTH]) -> Self {
        Self::from(c)
    }
}

pub trait OuterProductExt<T> {
    fn outer_product(self) -> T;
}

impl OuterProductExt<Mat3> for Vec3 {
    fn outer_product(self) -> Mat3 {
        Mat3::from_cols_array(&[
            self.x * self.x,
            self.y * self.x,
            self.z * self.x,
            self.x * self.y,
            self.y * self.y,
            self.z * self.y,
            self.x * self.z,
            self.y * self.z,
            self.z * self.z,
        ])
    }
}

pub struct DynAxisIter<V: IsVec = UVec3>
where
    V::Component: Copy,
{
    axis: WorldAxis,
    reverse: bool,
    pos: V,
    to: Option<V::Component>,
}

impl<V: IsVec> DynAxisIter<V>
where
    V::Component: Copy,
{
    pub fn new(axis: WorldAxis, from: V, to: V::Component) -> Self {
        Self {
            axis,
            reverse: false,
            pos: from,
            to: Some(to),
        }
    }

    pub fn new_reverse(axis: WorldAxis, from: V, to: V::Component) -> Self {
        Self {
            axis,
            reverse: true,
            pos: from,
            to: Some(to),
        }
    }
}

impl<V: IsVec> Iterator for DynAxisIter<V>
where
    V::Component: Copy + PartialOrd,
    WorldAxis: AsVecT<V>,
    V: Index<WorldAxis, Output = V::Component>,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let to = self.to?;
        let at = self.pos[self.axis];

        if self.reverse {
            return if at >= to {
                let result = self.pos;
                self.pos = self.pos - self.axis.as_vec_t();
                if at == to {
                    self.to = None;
                }
                Some(result)
            } else {
                None
            };
        }

        if self.pos[self.axis] <= to {
            let result = self.pos;
            self.pos = self.pos + self.axis.as_vec_t();
            Some(result)
        } else {
            None
        }
    }
}

pub struct StaticAxisIter<Dir: IterDirection, V: IsVec = UVec3>
where
    V::Component: Copy,
{
    pos: V,
    to: Option<V::Component>,
    _phantom: PhantomData<Dir>,
}

impl<V, Dir: IterDirection> Iterator for StaticAxisIter<Dir, V>
where
    V: IsVec + From<IterDirUnit> + Index<WorldAxis, Output = V::Component>,
    V::Component: Copy + PartialOrd,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let to = self.to?;
        let at = Dir::component_of(self.pos);

        if Dir::INVERSE {
            return if at >= to {
                let result = self.pos;
                self.pos = self.pos + Dir::UNIT.into();
                if at == to {
                    self.to = None;
                }
                Some(result)
            } else {
                None
            };
        }

        if Dir::component_of(self.pos) <= to {
            let result = self.pos;
            self.pos = self.pos + Dir::UNIT.into();
            Some(result)
        } else {
            None
        }
    }
}

impl<V: IsVec, Dir: IterDirection> StaticAxisIter<Dir, V>
where
    V::Component: Copy,
{
    pub fn new(from: V, to: V::Component) -> Self {
        Self {
            pos: from,
            to: Some(to),
            _phantom: PhantomData,
        }
    }
}

/*
pub trait UniformComponents<const Length: usize> {
    type Component;
    const LEN: usize = Length;

    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a;
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a;

    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self;

    fn nth_component(&self, n: usize) -> Option<&Self::Component> {
        self.components().nth(n)
    }
    fn nth_component_mut(&mut self, n: usize) -> Option<&mut Self::Component> {
        self.components_mut().nth(n)
    }

    #[inline]
    fn apply_op<F: Fn(&Self::Component) -> Self::Component>(&self, op: F) -> Self
    where
        Self: Sized,
    {
        Self::from_component_iter(self.components().map(op))
    }
    #[inline]
    fn apply_binary_op<F: Fn(&Self::Component, &Self::Component) -> Self::Component>(
        &self,
        other: &Self,
        op: F,
    ) -> Self
    where
        Self: Sized,
    {
        Self::from_component_iter(
            self.components()
                .zip(other.components())
                .map(|(a, b)| op(a, b)),
        )
    }
}

macro_rules! coord_array {
    (2, $on: expr) => {
        [&$on.x, &$on.y]
    };
    (3, $on: expr) => {
        [&$on.x, &$on.y, &$on.z]
    };
    (4, $on: expr) => {
        [&$on.x, &$on.y, &$on.z, &$on.w]
    };
}

macro_rules! constr_from_iter {
    (2, $st: ty, $iter: expr) => {
        <$st>::new(
            $iter.next().expect("iterator too short"),
            $iter.next().expect("iterator too short"),
        )
    };
    (3, $st: ty, $iter: expr) => {
        <$st>::new(
            $iter.next().expect("iterator too short"),
            $iter.next().expect("iterator too short"),
            $iter.next().expect("iterator too short"),
        )
    };
    (4, $st: ty, $iter: expr) => {
        <$st>::new(
            $iter.next().expect("iterator too short"),
            $iter.next().expect("iterator too short"),
            $iter.next().expect("iterator too short"),
            $iter.next().expect("iterator too short"),
        )
    };
}

macro_rules! impl_uniform_components {
    ($v: ty => $size: tt * $t: ty) => {
        impl UniformComponents<$size> for $v {
            type Component = $t;
            #[inline]
            fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
                coord_array!($size, self).into_iter()
            }
            #[inline]
            fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
                unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }
                    .into_iter()
            }
            fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
                let mut components = components.into_iter();
                constr_from_iter!($size, $v, components)
            }
        }
    };
}

impl_uniform_components!(IVec4 => 4 * i32);
impl_uniform_components!(UVec4 => 4 * u32);
impl_uniform_components!( Vec4 => 4 * f32);
impl_uniform_components!(IVec3 => 3 * i32);
impl_uniform_components!(UVec3 => 3 * u32);
impl_uniform_components!( Vec3 => 3 * f32);
impl_uniform_components!(IVec2 => 2 * i32);
impl_uniform_components!(UVec2 => 2 * u32);
impl_uniform_components!( Vec2 => 2 * f32);
*/
