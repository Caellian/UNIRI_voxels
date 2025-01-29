use std::marker::PhantomData;

use maybe_owned::{MaybeOwned, MaybeOwnedMut};
use once_cell::sync::Lazy;

use crate::math::side::Side;
use crate::math::vec::{IsVec, Mat3, UVec2, UVec3, Vec3, Vec3Swizzles};
use crate::world::chunk::{CHUNK_FRONT, ChunkStore, ChunkValueIndex, SizedGrid, SizedGridMut};

/// Used to rotate [`ChunkStore`] indexing.
#[derive(Clone)]
pub struct SideView<'d, T: PartialEq + 'd, G: SizedGrid<'d, T>> {
    inner: MaybeOwned<'d, G>,
    transform: (Mat3, Vec3),
    _phantom: PhantomData<T>,
}

pub(crate) fn wrapped_rows(mul: Mat3) -> (Mat3, Vec3) {
    // FIXME: unused row
    let cin = |row| {
        if mul.row(0).sum() < 0. {
            1.
        } else {
            0.
        }
    };
    (mul, Vec3::new(cin(0), cin(1), cin(2)))
}

pub(crate) static SIDE_VIEW_TRANSFORMS: Lazy<[(Mat3, Vec3); 6]> = Lazy::new(|| {
    Side::ALL
        .map(|it| CHUNK_FRONT.rotation_to(it))
        .map(wrapped_rows)
});

impl<'d, T: PartialEq + 'd, G: SizedGrid<'d, T>> SideView<'d, T, G> {
    pub fn new(inner: impl Into<MaybeOwned<'d, G>>, side: Side) -> Self {
        let inner = inner.into();
        let size = inner.size();
        Self {
            inner,
            transform: unsafe {
                // SAFETY: SIDE_VIEW_TRANSFORMS maps Side::ALL so every side index will yield a value
                let result = *SIDE_VIEW_TRANSFORMS.get_unchecked(side as usize);
                (result.0, result.1 * (size - UVec3::ONE).as_vec3())
            },
            _phantom: PhantomData,
        }
    }

    pub fn transform_position(&self, pos: UVec3) -> UVec3 {
        (self.transform.0 * pos.as_vec3() + self.transform.1).as_uvec3()
    }

    pub fn inverse_position(&self, pos: UVec3) -> UVec3 {
        (self.transform.0 * (pos.as_vec3() - self.transform.1)).as_uvec3()
    }
}

impl<'d, T: PartialEq + 'd, G: SizedGrid<'d, T>> SizedGrid<'d, T> for SideView<'d, T, G> {
    #[inline(always)]
    fn size(&self) -> UVec3 {
        self.inner.size()
    }
    #[inline(always)]
    fn get_position_index(&self, pos: UVec3) -> usize {
        self.inner.get_position_index(self.transform_position(pos))
    }
    #[inline(always)]
    fn get_pos_key(&self, pos: UVec3) -> Option<ChunkValueIndex> {
        self.inner.get_pos_key(pos)
    }
    #[inline(always)]
    fn values(&self) -> Vec<&'d T> {
        G::values(self.inner.as_ref())
    }
}

/// Used to slice into a [`ChunkStore`], yielding a 2D array.
///
/// Combine with [`SideView`] to allow slicing into arbitrary chunk sides.
#[derive(Clone)]
pub struct SliceView<'d, T: PartialEq + 'd, G: SizedGrid<'d, T>> {
    inner: MaybeOwned<'d, G>,
    depth: u32,
    _phantom: PhantomData<T>,
}

impl<'d, T: PartialEq, G: SizedGrid<'d, T>> SliceView<'d, T, G> {
    pub fn new(inner: impl Into<MaybeOwned<'d, G>>, depth: u32) -> Self {
        let inner = inner.into();
        if depth >= inner.size().z {
            panic!("depth is out of bounds");
        }
        SliceView {
            inner,
            depth,
            _phantom: PhantomData,
        }
    }
    pub fn try_new(inner: impl Into<MaybeOwned<'d, G>>, depth: i32) -> Option<Self> {
        let inner = inner.into();
        if depth > 0 && depth < inner.size().z as i32 {
            Some(SliceView {
                inner,
                depth: depth as u32,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn size(&self) -> UVec2 {
        self.inner.size().xy()
    }

    #[must_use]
    pub fn get_position_index(&self, pos: UVec2) -> usize {
        pos.x as usize
            + self.depth as usize * self.inner.size().x as usize
            + pos.y as usize * self.inner.size().x as usize * self.inner.size().z as usize
    }

    #[must_use]
    #[inline(always)]
    pub fn get_pos_key(&self, pos: UVec2) -> Option<ChunkValueIndex> {
        self.inner.get_pos_key(pos.extend(self.depth))
    }
    #[must_use]
    #[inline(always)]
    pub fn values(&self) -> Vec<&'d T> {
        G::values(self.inner.as_ref())
    }
    #[must_use]
    pub fn index_of_value(&self, value: &T) -> Option<u16>
    where
        T: PartialEq,
    {
        self.values()
            .iter()
            .enumerate()
            .take(u16::MAX as usize - 1)
            .find_map(|(i, it)| {
                if *it == value {
                    Some(i as u16 + 1)
                } else {
                    None
                }
            })
    }

    #[must_use]
    pub fn get_pos_value(&self, pos: UVec2) -> Option<&'d T> {
        if let Some(i) = self.get_pos_key(pos) {
            self.values().get(i as usize - 1).map(|it| *it)
        } else {
            None
        }
    }
}

/// Used to mutably slice into a [`ChunkStore`], yielding a 2D array.
///
/// Combine with [`SideView`] to allow slicing into arbitrary chunk sides.
pub struct SliceViewMut<'d, T: PartialEq + 'd, G: SizedGridMut<'d, T>> {
    inner: MaybeOwnedMut<'d, G>,
    depth: u32,
    _phantom: PhantomData<T>,
}

impl<'d, T: PartialEq + 'd, G: SizedGridMut<'d, T>> SliceViewMut<'d, T, G> {
    pub fn new(inner: impl Into<MaybeOwnedMut<'d, G>>, depth: u32) -> Self {
        let inner = inner.into();
        if depth >= inner.size().z {
            panic!("depth is out of bounds");
        }
        SliceViewMut {
            inner,
            depth,
            _phantom: PhantomData,
        }
    }
    pub fn try_new(inner: impl Into<MaybeOwnedMut<'d, G>>, depth: u32) -> Option<Self> {
        let inner = inner.into();
        if depth < inner.size().z {
            Some(SliceViewMut {
                inner,
                depth,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn size(&self) -> UVec2 {
        self.inner.size().xy()
    }

    #[must_use]
    pub fn get_position_index(&self, pos: UVec2) -> usize {
        pos.x as usize
            + self.depth as usize * self.inner.size().x as usize
            + pos.y as usize * self.inner.size().x as usize * self.inner.size().z as usize
    }

    #[must_use]
    #[inline(always)]
    pub fn get_pos_key(&self, pos: UVec2) -> Option<ChunkValueIndex> {
        self.inner.get_pos_key(pos.extend(self.depth))
    }
    #[must_use]
    #[inline(always)]
    pub fn get_pos_key_mut(&mut self, pos: UVec2) -> Option<&mut ChunkValueIndex> {
        self.inner.get_pos_key_mut(pos.extend(self.depth))
    }
    #[must_use]
    #[inline(always)]
    pub fn get_values(&self) -> Vec<&'d T> {
        G::values(self.inner.as_ref())
    }
    #[must_use]
    #[inline(always)]
    pub fn value_list_mut(&mut self) -> &mut Vec<T> {
        self.inner.value_list_mut()
    }
    #[inline(always)]
    pub fn insert_key(&mut self, key: T) -> u16 {
        self.inner.insert_key(key)
    }
    #[must_use]
    pub fn index_of_value(&self, value: &T) -> Option<u16>
    where
        T: PartialEq,
    {
        self.get_values()
            .iter()
            .enumerate()
            .take(u16::MAX as usize - 1)
            .find_map(|(i, it)| {
                if *it == value {
                    Some(i as u16 + 1)
                } else {
                    None
                }
            })
    }

    #[inline]
    pub fn set_pos_id(&mut self, pos: UVec2, value: ChunkValueIndex) {
        if let Some(pos) = self.get_pos_key_mut(pos) {
            *pos = value;
        }
    }

    #[must_use]
    pub fn get_pos_value(&self, pos: UVec2) -> Option<&'d T> {
        if let Some(i) = self.get_pos_key(pos) {
            self.get_values().get(i as usize - 1).map(|it| *it)
        } else {
            None
        }
    }

    pub fn set_pos_value(&mut self, pos: UVec2, value: Option<T>)
    where
        T: PartialEq,
    {
        if pos.x >= self.size().x || pos.y >= self.size().y {
            return;
        }
        let i = value
            .map(|it| {
                self.index_of_value(&it).unwrap_or_else(|| {
                    self.value_list_mut().push(it);
                    self.get_values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }

    pub fn set_or_clone_pos_value(&mut self, pos: UVec2, value: Option<&T>)
    where
        T: PartialEq + Clone,
    {
        let i = value
            .map(|it| {
                self.index_of_value(it).unwrap_or_else(|| {
                    self.value_list_mut().push(it.clone());
                    self.get_values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }
}
