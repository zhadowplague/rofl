use std::fs::File;
use std::io::{self, prelude::*, BufReader, Stdout};
use std::path::PathBuf;
use crossterm::{
    QueueableCommand,
    cursor, style::{self}
};
use vector2d::Vector2D;

pub struct Sprite {
    character_rows : Vec<Vec<String>>
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
        return Ok(Sprite{ character_rows });
    }

    pub fn draw(&self, current_frame:usize, mut stdout_handle:&Stdout, translation:&Vector2D<u16>) {
        for line in &self.character_rows[current_frame] {
            let _ = stdout_handle.queue(cursor::MoveTo(translation.x, translation.y));
            let _ = stdout_handle.queue(style::Print(line));
        }
    }
}