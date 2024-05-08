use vector2d::Vector2D;
use std::fs::File;
use std::path::PathBuf;
use std::io::{self, prelude::*, BufReader};

pub struct EnemyData {
    size : Vector2D<usize>,
    frames_per_second : usize,
    damage : usize,
    velocity : Vector2D<f32>
}

pub struct Enemy {
    pub translation : Vector2D<u16>,
    pub texture_index : usize,
    pub current_frame : usize,
    current_frame_counter : f32,
    translation_counter : Vector2D<f32>,
    data : EnemyData
}

impl Enemy {
    pub fn load(path:&PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let mut data = EnemyData {damage : 0, frames_per_second : 0, size : Vector2D::new(0, 0), velocity : Vector2D::new(0.0, 0.0)};
        let mut i = 0;
        for line in reader.lines() {
            let unwraped_line = line?;
            match i {
                1 => data.damage = unwraped_line.parse()?,
                _ => return Err("Bad file".into())
            }
            i += 1;
        }        
        return Ok(Enemy{ translation : Vector2D::new(0, 0), current_frame : 0, current_frame_counter : 0.0, texture_index : 0, translation_counter : Vector2D { x: 0.0, y: 0.0 }, data });
    }

    pub fn update(&mut self, delta:f32) {        
        self.current_frame_counter += delta * self.data.frames_per_second as f32;
        self.translation_counter += self.data.velocity * delta;

        if self.current_frame_counter as usize >= 1 {
            self.current_frame_counter = 0.0;
            self.current_frame += 1;
        }

        if self.translation_counter.x as usize >= 1 {
            self.translation_counter.x = 0.0;
            self.translation.x += 1;
        }

        if self.translation_counter.y as usize >= 1 {
            self.translation_counter.y = 0.0;
            self.translation.y += 1;
        }
    }
}
