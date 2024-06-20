use vector2d::Vector2D;
use crate::{data::sprite::Sprite, utils};
use crossterm::{
    QueueableCommand,
    cursor, style::{self}
};
use std::io::Stdout;

pub fn draw_sprite(sprite:&Sprite, frame:usize, translation:&Vector2D<f32>, mut stdout_handle:&Stdout, game_width:f32) {
    let frame_count = sprite.frames.len();
    let active_frame = utils::normalize_usize(0, frame_count - 1, frame);
    let mut row :u16 = 0;
    let (x,_y) = (translation.x as u16, translation.y as u16);
    for line in sprite.frames[active_frame].iter() {
        let _ = stdout_handle.queue(cursor::MoveTo(translation.x as u16, translation.y as u16 + row));
        if x < game_width as u16 {
            let overflow = (x + sprite.max_width as u16).checked_sub(game_width as u16);
            if overflow.is_some_and(|x| x > 0) {
                let (_first_line, second_line) = line.split_at(overflow.unwrap() as usize);
                let _ = stdout_handle.queue(style::Print(second_line));
            } else {
                let _ = stdout_handle.queue(style::Print(line));
            } 
        }
        row += 1;
    }
}