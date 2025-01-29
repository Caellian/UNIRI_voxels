use serde::{Deserialize, Serialize};

use crate::math::vec::IsVec;

pub trait Contains<T> {
    fn test_contains(&self, value: &T) -> bool;
}

pub trait Intersects<T> {
    fn test_intersects(&self, value: &T) -> bool;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AABB<T> {
    /// Inclusive lower bound
    pub start: T,
    /// Inclusive upper bound
    pub end: T,
}

impl<T: IsVec> AABB<T>
where
    T::Component: std::fmt::Debug,
    [T::Component; T::LENGTH]: Sized,
{
    pub fn new(start: T, end: T) -> Self {
        let (start, end): (Vec<T::Component>, Vec<T::Component>) = start
            .components()
            .into_iter()
            .zip(end.components().into_iter())
            .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
            .unzip();
        let (start, end): ([T::Component; T::LENGTH], [T::Component; T::LENGTH]) = unsafe {
            (
                start.try_into().unwrap_unchecked(),
                end.try_into().unwrap_unchecked(),
            )
        };
        AABB {
            start: T::from_components(start),
            end: T::from_components(end),
        }
    }

    pub fn new_unchecked(start: T, end: T) -> Self {
        AABB { start, end }
    }

    pub fn center(&self) -> T {
        self.start + self.end / <T::Component as num::cast::NumCast>::from(2).unwrap()
    }
}

impl Contains<ChunkPos> for AABB<ChunkPos> {
    fn test_contains(&self, value: &ChunkPos) -> bool {
        let v = value.value;
        let s = self.start.value;
        let e = self.end.value;
        v.x >= s.x && v.y >= s.y && v.z >= s.z && v.x <= e.x && v.y <= e.y && v.z <= e.z
    }
}

impl Intersects<ChunkPos> for AABB<ChunkPos> {
    fn test_intersects(&self, value: &ChunkPos) -> bool {
        todo!()
    }
}
