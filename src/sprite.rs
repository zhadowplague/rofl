use std::fs::File;
use std::io::{self, prelude::*, BufReader, Stdout};
use std::path::PathBuf;
use crossterm::{
    QueueableCommand,
    cursor, style::{self}
};
use vector2d::Vector2D;

pub struct Sprite {
    frames : Vec<Vec<String>>
}

pub fn draw_sprite(sprite:&Sprite, frame:usize, translation:&Vector2D<f32>, mut stdout_handle:&Stdout) {
    for line in sprite.frames[frame].iter() {
        let _ = stdout_handle.queue(cursor::MoveTo(translation.x as u16, translation.y as u16));
        let _ = stdout_handle.queue(style::Print(line));
    }
}

impl Sprite {
    pub fn load(path:&PathBuf) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let mut character_rows = Vec::<Vec<String>>::new();
        character_rows.push(Vec::<String>::new());
        for line in reader.lines() {
            let unwraped_line = line?;
            if unwraped_line.contains("framedivider") {
                character_rows.push(Vec::<String>::new());
                continue;
            }
            let current_row = character_rows.len() - 1;
            character_rows[current_row].push(unwraped_line);
        }
        return Ok(Sprite{ frames: character_rows });
    }
}