use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub struct Sprite {
    pub frames : Vec<Vec<String>>,
    pub max_width : u16
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
        let mut max_width = 0;
        for line in character_rows.iter() {
            let len = line.len();
            if len > max_width {
                max_width = len;
            }
        }
        return Ok(Sprite{ frames: character_rows, max_width : max_width as u16 });
    }
}