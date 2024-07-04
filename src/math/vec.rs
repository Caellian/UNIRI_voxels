use std::{cmp::Reverse, marker::PhantomData, ops::Index};

use bevy::math::*;

use crate::{convert::Convert, world::WorldAxis};

pub struct X;
pub struct Y;
pub struct Z;

pub trait IterDirection {
    const UNIT: IVec3;
    const AXIS: WorldAxis;
    const INVERSE: bool = false;

    #[inline(always)]
    fn component_of<V: IsVec<3>>(v: V) -> V::Component {
        v.axis_component(Self::AXIS)
    }

    #[inline(always)]
    fn zero<V: IsVec<3>>() -> V::Component
    where
        IVec3: Convert<V>,
        V: Index<usize, Output = V::Component>,
    {
        IVec3::ZERO.convert()[0]
    }
}
impl IterDirection for X {
    const UNIT: IVec3 = IVec3::X;
    const AXIS: WorldAxis = WorldAxis::X;
}
impl IterDirection for Y {
    const UNIT: IVec3 = IVec3::Y;
    const AXIS: WorldAxis = WorldAxis::Y;
}
impl IterDirection for Z {
    const UNIT: IVec3 = IVec3::Z;
    const AXIS: WorldAxis = WorldAxis::Z;
}
impl IterDirection for Reverse<X> {
    const UNIT: IVec3 = IVec3::new(-1, 0, 0);
    const AXIS: WorldAxis = WorldAxis::X;
    const INVERSE: bool = true;
}
impl IterDirection for Reverse<Y> {
    const UNIT: IVec3 = IVec3::new(0, -1, 0);
    const AXIS: WorldAxis = WorldAxis::Y;
    const INVERSE: bool = true;
}
impl IterDirection for Reverse<Z> {
    const UNIT: IVec3 = IVec3::new(0, 0, -1);
    const AXIS: WorldAxis = WorldAxis::Z;
    const INVERSE: bool = true;
}

pub trait IsVec<const Length: usize>:
    Sized
    + UniformComponents<Length>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + Clone
    + Copy
{
    #[inline]
    fn axis_component(&self, axis: WorldAxis) -> Self::Component {
        *self.nth_component(axis as usize)
    }

    #[inline]
    fn sum(self) -> Self::Component
    where
        Self::Component: std::iter::Sum<Self::Component>,
    {
        self.components().sum()
    }

    #[inline]
    fn inner_product(self) -> Self::Component
    where
        Self::Component: std::ops::Mul<Output = Self::Component> + std::iter::Sum<Self::Component>,
    {
        self.components().map(|it| it * it).sum()
    }
}

impl IsVec<4> for UVec4 {}
impl IsVec<4> for IVec4 {}
impl IsVec<4> for Vec4 {}
impl IsVec<3> for UVec3 {}
impl IsVec<3> for IVec3 {}
impl IsVec<3> for Vec3 {}
impl IsVec<2> for UVec2 {}
impl IsVec<2> for IVec2 {}
impl IsVec<2> for Vec2 {}

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

pub struct DynAxisIter<V: IsVec<3> = UVec3> {
    axis: WorldAxis,
    reverse: bool,
    pos: V,
    to: Option<V::Component>,
}

impl<V: IsVec<3>> DynAxisIter<V> {
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

impl<V: IsVec<3>> Iterator for DynAxisIter<V>
where
    Vec3: Convert<V>,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let to = self.to?;
        let at = self.pos.axis_component(self.axis);

        if self.reverse {
            return if at >= to {
                let result = self.pos;
                self.pos = self.pos - self.axis.to_vec().convert();
                if at == to {
                    self.to = None;
                }
                Some(result)
            } else {
                None
            };
        }

        if self.pos.axis_component(self.axis) <= to {
            let result = self.pos;
            self.pos = self.pos + self.axis.to_vec().convert();
            Some(result)
        } else {
            None
        }
    }
}

pub struct StaticAxisIter<Dir: IterDirection, V: IsVec<3> = UVec3> {
    pos: V,
    to: Option<V::Component>,
    _phantom: PhantomData<Dir>,
}

impl<V: IsVec<3>, Dir: IterDirection> StaticAxisIter<Dir, V> {
    pub fn new(from: V, to: V::Component) -> Self {
        Self {
            pos: from,
            to: Some(to),
            _phantom: PhantomData,
        }
    }
}

impl<V: IsVec<3>, Dir: IterDirection> Iterator for StaticAxisIter<Dir, V>
where
    IVec3: Convert<V>,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let to = self.to?;
        let at = Dir::component_of(self.pos);

        if Dir::INVERSE {
            return if at >= to {
                let result = self.pos;
                self.pos = self.pos + Dir::UNIT.convert();
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
            self.pos = self.pos + Dir::UNIT.convert();
            Some(result)
        } else {
            None
        }
    }
}
pub trait UniformComponents<const Length: usize> {
    type Component;
    const LEN: usize = Length;

    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a;
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a;

    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self;

    fn nth_component(&self, n: usize) -> &Self::Component {
        self.components().nth(n)
    }
    fn nth_component_mut(&mut self, n: usize) -> &mut Self::Component {
        self.components_mut().nth(n)
    }

    #[inline]
    fn apply_op<F: Fn(&Self::Component) -> Self::Component>(&self, op: F) -> Self {
        Self::from_component_iter(self.components().map(op))
    }
    #[inline]
    fn apply_binary_op<F: Fn(&Self::Component, &Self::Component) -> Self::Component>(
        &self,
        other: &Self,
        op: F,
    ) -> Self {
        Self::from_component_iter(self.components().zip(other.components()).map(op))
    }
}

impl UniformComponents<4> for IVec4 {
    type Component = i32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y, &self.z, &self.w].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<4> for UVec4 {
    type Component = u32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y, &self.z, &self.w].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<4> for Vec4 {
    type Component = f32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y, &self.z, &self.w].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<3> for IVec3 {
    type Component = i32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y, &self.z].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<3> for UVec3 {
    type Component = u32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y, &self.z].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<3> for Vec3 {
    type Component = f32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y, &self.z].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<2> for IVec2 {
    type Component = i32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<2> for UVec2 {
    type Component = u32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}

impl UniformComponents<2> for Vec2 {
    type Component = f32;

    #[inline]
    fn components<'a>(&'a self) -> impl Iterator<Item = &Self::Component> + 'a {
        [&self.x, &self.y].into_iter()
    }
    #[inline]
    fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Self::Component> + 'a {
        unsafe { std::mem::transmute::<_, &mut [Self::Component; Self::LEN]>(self) }.into_iter()
    }
    fn from_component_iter(components: impl IntoIterator<Item = Self::Component>) -> Self {
        let mut components = components.into_iter();
        Self::new(
            components.next().expect("iterator too short"),
            components.next().expect("iterator too short"),
        )
    }
}
