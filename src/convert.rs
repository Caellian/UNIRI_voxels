use crate::math::vec::UniformComponents;

use tt_call::tt_if;
use tt_equal::tt_equal;

/// Mimics [`From`] and [`Into`], but allows implementation for external types.
pub trait Convert<Into> {
    fn convert(self) -> Into;
}

/// Any type can be converted into itself.
impl<T> Convert<T> for T {
    #[inline(always)]
    fn convert(self) -> T {
        self
    }
}

macro_rules! prim_conv_impl {
    ($a: ty, $b: ty) => {
        tt_if! {
            condition = [{tt_equal}]
            input = [{ $a $b }]
            true = [{}]
            false = [{
                impl Convert<$b> for $a {
                    #[inline(always)]
                    fn convert(self) -> $b {
                        self as $b
                    }
                }
            }]
        }
    };
}
macro_rules! cross_prod_call {
    ($invoke: ident, [], [$(,)?$($b_rest: ty),*], [$($carry: ty),+]) => {};
    ($invoke: ident, [$a: ty $(,$a_rest: ty)*], [], [$($carry: ty),+]) => {
        cross_prod_call!($invoke, [$($a_rest),*], [$($carry),+], [$($carry),+]);
    };
    ($invoke: ident, [$a: ty $(,$a_rest: ty)*], [$b: ty $(,$b_rest: ty)*], [$($carry: ty),+]) => {
        $invoke!($a, $b);
        cross_prod_call!($invoke, [$a$(,$a_rest)*], [$($b_rest),*], [$($carry),+]);
    };
    ($invoke: ident, $($a: ty),+) => {
        cross_prod_call!($invoke, [$($a),+], [$($a),+], [$($a),+]);
    };
}
cross_prod_call!(
    prim_conv_impl,
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    f32,
    f64
);

impl Convert<bevy::render::color::Color> for dot_vox::Color {
    fn convert(self) -> bevy::render::color::Color {
        bevy::render::color::Color::Rgba {
            red: self.r as f32 / 255.0,
            green: self.g as f32 / 255.0,
            blue: self.b as f32 / 255.0,
            alpha: self.a as f32 / 255.0,
        }
    }
}

impl Convert<wgpu::Color> for dot_vox::Color {
    fn convert(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64 / 255.0,
            g: self.g as f64 / 255.0,
            b: self.b as f64 / 255.0,
            a: self.a as f64 / 255.0,
        }
    }
}

impl<AC, A, BC, B, const L: usize> Convert<B> for A
where
    A: UniformComponents<L, Component = AC>,
    B: UniformComponents<L, Component = BC>,
    AC: Convert<BC> + Clone + Copy,
{
    fn convert(self) -> B {
        B::from_component_iter(self.components().map(|it| (*it).convert()))
    }
}
