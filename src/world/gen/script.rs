use bevy::math::IVec3;
use noise::{Fbm, NoiseFn, Simplex};

pub fn noise_test(chunk_pos: IVec3, surface_height: usize, size: usize) {
    let fbm: Fbm<Simplex> = Fbm::new(2343536);

    for z in 0..size {
        for x in 0..size {
            let precise_height = fbm.get([
                chunk_pos.x as f64 * size as f64 + x as f64,
                chunk_pos.z as f64 * size as f64 + z as f64,
            ]);
            let height = precise_height;
            println!("{:?} height: {}", (x, z), height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise() {
        let d = noise_test(IVec3::new(0, 0, 0), 0, 32);
    }
}
