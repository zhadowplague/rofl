use vector2d::Vector2D;
use crate::utils;

pub struct EnemyData {
    pub texture_index : usize,
    pub current_frame : f32,
    pub translation : Vector2D<f32>,
    pub damage : usize,
    pub health : usize
}

impl EnemyData {
    pub fn new(elapsed_seconds:u64, screen_size : &Vector2D<u16>, texture_index:usize, texture_height:usize) -> Self {
        const MAX_STRENGTH_AT_SECONDS: f32 = 300.0;
        const MAX_STRENGTH: f32 = 30.0;
        let strength = f32::min(f32::max(elapsed_seconds as f32, 0.1), MAX_STRENGTH_AT_SECONDS) / 10.0;
        let damage = f32::max(utils::normalize_f32(0.0, MAX_STRENGTH, strength), 1.0) as usize;
        let health = (texture_height as f32 * 0.5) as usize;

        let y_pos = rand::random::<u16>() % screen_size.y.checked_sub(texture_height as u16).unwrap();
        let translation = Vector2D::new(screen_size.x as f32, y_pos as f32);
        let current_frame = 0.0;
        return EnemyData{ current_frame, texture_index, translation, damage, health };
    }
}
