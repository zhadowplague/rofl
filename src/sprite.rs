use std::fs::File;
use std::io::{self, prelude::*, BufReader, Stdout};
use crossterm::{
    QueueableCommand,
    cursor, style::{self}
};

pub struct Translation {
    pub pos_x : u16,
    pub pos_y : u16,
}

pub struct Sprite{
    translation : Translation,
    character_rows : Vec<Vec<String>>,
    frames_per_second : usize,
    current_frame : f32,
    stdout_handle : Stdout,
}

impl Sprite {
    pub fn load(path:&str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut character_rows = Vec::<Vec<String>>::new();
        character_rows.push(Vec::<String>::new());
        for line in reader.lines() {
            let unwraped_line = line.unwrap();
            if unwraped_line.contains("framedivider") {
                character_rows.push(Vec::<String>::new());
                continue;
            }
            let current_row = character_rows.len() - 1;
            character_rows[current_row].push(unwraped_line);
        }
        let stdout_handle = io::stdout();
        let translation = Translation { pos_x:0, pos_y:0 };
        return Sprite{ character_rows, frames_per_second : 1, stdout_handle, translation, current_frame : 0.0 };
    }

    pub fn draw(&mut self, delta:f32) {
        let current_frame : usize = self.current_frame as usize;
        for line in &self.character_rows[current_frame] {
            let _ = self.stdout_handle.queue(cursor::MoveTo(self.translation.pos_x, self.translation.pos_y));
            let _ = self.stdout_handle.queue(style::Print(line));
        }
        self.current_frame += delta * self.frames_per_second as f32;
    }
}