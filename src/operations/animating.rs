use crate::data::enemy::EnemyData;
use lerp::Lerp;

pub fn animate(enemies:&mut Vec<EnemyData>, delta:f32) {
    for enemy in enemies.iter_mut() {
        let frames_per_second = 1.0.lerp(6.0, enemy.normalized_strength) as usize;
        enemy.current_frame += delta * frames_per_second as f32;
    }
}