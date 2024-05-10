use vector2d::Vector2D;
use crate::data::sprite::Sprite;
use crossterm::{
    QueueableCommand,
    cursor, style::{self}
};
use std::io::Stdout;

pub fn draw_sprite(sprite:&Sprite, frame:usize, translation:&Vector2D<f32>, mut stdout_handle:&Stdout) {
    for line in sprite.frames[frame].iter() {
        let _ = stdout_handle.queue(cursor::MoveTo(translation.x as u16, translation.y as u16));
        let _ = stdout_handle.queue(style::Print(line));
    }
}