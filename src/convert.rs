use bevy::color::Srgba;

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

impl Convert<bevy::color::Color> for dot_vox::Color {
    fn convert(self) -> bevy::color::Color {
        bevy::color::Color::Srgba(Srgba {
            red: self.r as f32 / 255.0,
            green: self.g as f32 / 255.0,
            blue: self.b as f32 / 255.0,
            alpha: self.a as f32 / 255.0,
        })
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
