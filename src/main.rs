use std::io::{self, Write};
use std::time::Duration;
use std::fs;
use std::time::Instant;
use crossterm::event::{poll, read, KeyCode};
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
  let mut enemies = load_enemies()?;
  let mut delta = 0.0;
  
  //1. Gather user input
  let mut keystrokes = Vec::<KeyCode>::new();
  while poll(Duration::from_secs(0)).is_ok() {
    match read()? {
        crossterm::event::Event::Key(key_event) => 
          if matches!(key_event.kind, crossterm::event::KeyEventKind::Press) {
            keystrokes.push(key_event.code);
          },
        _ => ()
    }
  }
  //2. Update state
  let start = Instant::now();
  for enemy in enemies.iter_mut() {
    enemy.update(delta);
  }
  delta += start.elapsed().as_millis() as f32 * 0.001;

  //3, Draw state to screen
  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for enemy in enemies.iter() {
    let sprite = &sprites[enemy.texture_index];
    sprite.draw(enemy.current_frame as usize, &stdout, &enemy.translation);
  }

  stdout.flush()?;
  Ok(())
}