use crate::data::enemy::EnemyData;

pub fn move_straight(enemies:&mut Vec<EnemyData>, delta:f32) {
    for enemy in enemies.iter_mut() {
        enemy.translation.x -= delta * 2.0;
    }
}