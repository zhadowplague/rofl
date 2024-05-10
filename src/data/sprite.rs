use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub struct Sprite {
    pub frames : Vec<Vec<String>>
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