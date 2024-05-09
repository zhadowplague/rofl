use lerp::Lerp;
use vector2d::Vector2D;
use crate::utils;

pub struct Enemy {
    pub translation : Vector2D<u16>,
    pub texture_index : usize,
    pub current_frame : usize,
    current_frame_counter : f32,
    translation_counter : Vector2D<f32>,
    frames_per_second : usize,
    pub damage : usize,
    velocity : Vector2D<f32>
}

impl Enemy {
    pub fn new(elapsed_seconds:u64) -> Self {
        const MAX_STRENGTH_AT_SECONDS: f32 = 300.0;
        const MAX_STRENGTH: f32 = 30.0;
        let strength = f32::min(f32::max(elapsed_seconds as f32, 0.1), MAX_STRENGTH_AT_SECONDS) / 10.0;
        let normalized_strength = utils::normalize(0.0, MAX_STRENGTH, strength);

        let damage = f32::max(strength, 1.0) as usize;
        let translation = Vector2D::new(0, 0);
        let translation_counter = Vector2D::new(0.0, 0.0);
        let current_frame = 0;
        let texture_index = 0;
        let current_frame_counter = 0.0;
        let frames_per_second = 1.0.lerp(6.0, normalized_strength) as usize;
        let velocity = Vector2D::new(0.0, 0.0);
        return Enemy{ translation, current_frame, current_frame_counter, texture_index, translation_counter, damage, frames_per_second, velocity };
    }

    pub fn update(&mut self, delta:f32) {        
        self.current_frame_counter += delta * self.frames_per_second as f32;
        self.translation_counter += self.velocity * delta;

        if self.current_frame_counter as usize >= 1 {
            self.current_frame_counter = 0.0;
            self.current_frame += 1;
        }

        if self.translation_counter.x as usize >= 1 {
            self.translation_counter.x = 0.0;
            self.translation.x += 1;
        }

        if self.translation_counter.y as usize >= 1 {
            self.translation_counter.y = 0.0;
            self.translation.y += 1;
        }
    }
}
