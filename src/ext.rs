use bevy::prelude::{UVec3, Vec3};

use crate::world::WorldAxis;

pub trait Convert<Into> {
    fn convert(self) -> Into;
}

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

impl Convert<bevy::math::Vec3> for bevy::math::UVec3 {
    fn convert(self) -> bevy::math::Vec3 {
        bevy::math::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl Convert<bevy::math::UVec3> for bevy::math::Vec3 {
    fn convert(self) -> bevy::math::UVec3 {
        bevy::math::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }
}

pub trait VecExt<T> {
    const DIM: usize;

    fn get_axis_coord(&self, axis: WorldAxis) -> Option<T>;
    fn sum(&self) -> T;
}

impl VecExt<u32> for UVec3 {
    const DIM: usize = 3;

    fn get_axis_coord(&self, axis: WorldAxis) -> Option<u32> {
        match axis {
            WorldAxis::X => Some(self.x),
            WorldAxis::Y => Some(self.y),
            WorldAxis::Z => Some(self.z),
        }
    }

    fn sum(&self) -> u32 {
        self.x + self.y + self.z
    }
}

impl VecExt<f32> for Vec3 {
    const DIM: usize = 3;

    fn get_axis_coord(&self, axis: WorldAxis) -> Option<f32> {
        match axis {
            WorldAxis::X => Some(self.x),
            WorldAxis::Y => Some(self.y),
            WorldAxis::Z => Some(self.z),
        }
    }

    fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }
}
