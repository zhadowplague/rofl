use std::io::{self, Write};
use std::fs;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

mod sprite;

const SPRITE_FOLDER : &str = "sprites";

fn load_sprites() -> Result<Vec<sprite::Sprite>, io::Error> {
  let mut sprites = Vec::<sprite::Sprite>::new();
  let mut path = std::env::current_exe()?;
  path.push(SPRITE_FOLDER);

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

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  let sprites = load_sprites()?;

  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        stdout
          .queue(cursor::MoveTo(x,y))?
          .queue(style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }

  for mut sprite in sprites {
    sprite.draw(0.0);
  }

  stdout.flush()?;
  Ok(())
}