use std::io::{self, Write};
use std::fs;
use crossterm::{
    ExecutableCommand,
    terminal,
};

mod sprite;
mod enemy;

#[cfg(not(debug_assertions))]
const SPRITE_FOLDER : &str = "sprites";
#[cfg(debug_assertions)]
const SPRITE_FOLDER_DEBUG : &str = "..\\..\\..\\sprites";

#[cfg(debug_assertions)]
fn sprite_folder_path() -> &'static str{
  return SPRITE_FOLDER_DEBUG
}

#[cfg(not(debug_assertions))]
fn sprite_folder_path() -> &'static str{
  return SPRITE_FOLDER;
}

fn load_sprites() -> Result<Vec<sprite::Sprite>, io::Error> {
  let mut sprites = Vec::<sprite::Sprite>::new();
  let mut path = std::env::current_exe()?;
  path.push(sprite_folder_path());

  let dir = fs::read_dir(path)?;
  for entry in dir {
    let u_entry = entry?;
    if u_entry.metadata()?.is_file() {
      let sprite = sprite::Sprite::load(&u_entry.path())?;
      sprites.push(sprite);
    }
  }

  return Ok(sprites);
}

fn load_enemies() -> Result<Vec<enemy::Enemy>, io::Error> {
  let enemies = Vec::<enemy::Enemy>::new();
  return Ok(enemies);
}

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  let sprites = load_sprites()?;
  let enemies = load_enemies()?;
  let mut delta = 0.0;

  //1. Gather user input

  //2. Update state

  //3, Draw state to screen
  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for mut enemy in enemies {
    enemy.update(delta);
    let sprite = &sprites[enemy.texture_index];
    sprite.draw(enemy.current_frame as usize, &stdout, &enemy.translation);
  }

  stdout.flush()?;
  Ok(())
}