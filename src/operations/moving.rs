use crate::data::enemy::EnemyData;

pub fn move_straight(enemies: &mut[EnemyData], delta:f32) {
    for enemy in enemies {
        enemy.translation.x -= delta * 2.0;
    }
}

pub fn move_sin(enemies: &mut[EnemyData], delta:f32) {
    for enemy in enemies {
        enemy.translation.x -= delta * 2.0;
        let mod_x = enemy.translation.x as usize % 10;
        if mod_x >= 5 {
            enemy.translation.y += delta * 2.0;
        }
        else {
            enemy.translation.y -= delta * 2.0;
        }
    }
}