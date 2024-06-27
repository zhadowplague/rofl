use stack_vec::stack;
use vector2d::Vector2D;

use crate::{constants::PLAYER_SIZE, data::{enemy::EnemyData, sprite::Sprite}, utils::within};

pub fn remove_bullets_off_screen(bullets: &mut Vec<Vector2D<f32>>, game_width:u16) {
    let mut bl = stack![usize; 20];

    let mut i = 0;
    for bullet in bullets.iter() {
        if bullet.x > game_width as f32 {
            bl.push(i);
        }
        i += 1;
    }
    for r in bl.iter().rev() {
        bullets.swap_remove(*r);
    }
}

pub fn remove_enemy_bullets_under_collision(enemies:&mut Vec<EnemyData>, bullets: &mut Vec<Vector2D<f32>>, sprites: &Vec<Sprite>) {
    let mut en = stack![usize; 10];

    let mut i = 0;
    for enemy in enemies.iter_mut() {
        let enemy_sprite = &sprites[enemy.texture_index];
        if enemy.translation.x < PLAYER_SIZE {
            en.push(i);
        }
        else {
            let mut j = 0;
            let mut bl = stack![usize; 20];
            for bullet in bullets.iter() {
                if within(&enemy.translation, bullet, enemy_sprite.max_height, enemy_sprite.max_width) {
                    let new_health = enemy.health.checked_sub(1);
                    if new_health.is_none() {
                        en.push(i);
                    }
                    else {
                        enemy.health = new_health.unwrap();
                    }
                    bl.push(j);
                    break;
                }
                j += 1;
            }
            for r in bl.iter().rev() {
                bullets.swap_remove(*r);
            }
        }
        i += 1;
    }
    for r in en.iter().rev() {
        enemies.swap_remove(*r);
    }
}