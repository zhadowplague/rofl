use vector2d::Vector2D;
use crate::{data::sprite::Sprite, utils};
use crossterm::{
    QueueableCommand,
    cursor, style::{self}
};
use std::io::Stdout;

pub fn draw_sprite(sprite:&Sprite, frame:usize, translation:&Vector2D<f32>, mut stdout_handle:&Stdout, screen_size:&Vector2D<u16>) {
    let (screen_size_x, screen_size_y) = (screen_size.x - 1, screen_size.y);
    let frame_count = sprite.frames.len();
    let active_frame = utils::normalize_usize(0, frame_count - 1, frame);
    let mut row :u16 = 0;
    let (x,y) = (translation.x as u16, translation.y as u16);
    for line in sprite.frames[active_frame].iter() {
        if y + row > screen_size_y {
            continue;
        }
        if x < screen_size_x {
            let _ = stdout_handle.queue(cursor::MoveTo(x, y + row));
            let overflow = (x + sprite.max_width).checked_sub(screen_size_x);
            if overflow.is_some_and(|x| x > 0) {
                let (first_line, _second_line) = line.split_at(line.len() - overflow.unwrap() as usize);
                let _ = stdout_handle.queue(style::Print(first_line));
            } else {
                let _ = stdout_handle.queue(style::Print(line));
            } 
        }
        row += 1;
    }
}