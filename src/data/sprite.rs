use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub struct Sprite {
    pub frames : Vec<Vec<String>>,
    pub frame_lengths : Vec<Vec<usize>>,
    pub max_width : usize,
    pub max_height : usize
}

impl Sprite {
    pub fn load(path:&PathBuf) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let mut frame_lengths = Vec::<Vec<usize>>::new();
        let mut character_rows = Vec::<Vec<String>>::new();
        character_rows.push(Vec::new());
        for line in reader.lines() {
            let unwraped_line = line?;
            if unwraped_line.contains("framedivider") {
                character_rows.push(Vec::new());
                continue;
            }
            let current_row = character_rows.len() - 1;
            character_rows[current_row].push(unwraped_line);
        }
        let mut max_width = 0;
        let mut max_height = 0;
        for frame in character_rows.iter() {
            let mut frame_length: Vec<usize> = Vec::new();
            for line in frame {
                let len = line.char_indices().count();
                if len > max_width {
                    max_width = len;
                }
                frame_length.push(len);
            }
            let height = frame_length.len();
            if height > max_height {
                max_height = height;
            }
            frame_lengths.push(frame_length);
        }
        return Ok(Sprite{ frames: character_rows, frame_lengths, max_width, max_height });
    }
}