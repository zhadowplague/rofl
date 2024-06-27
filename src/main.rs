use std::io::{self, Write, Error, ErrorKind};
use std::time::Duration;
use std::fs;
use std::time::Instant;
use constants::{sprite_folder_path, MAX_BULLET_COUNT, MAX_ENEMY_COUNT, PLAYER_SIZE};
use crossterm::event::{poll, read, KeyCode, KeyModifiers};
use crossterm::style;
use crossterm::terminal::enable_raw_mode;
use crossterm::{ 
  execute, cursor, QueueableCommand, terminal, terminal::{SetSize, size}
};
use operations::collision::{remove_bullets_off_screen, remove_enemy_bullets_under_collision};
use utils::rand_range;
use vector2d::Vector2D;

mod data;
use crate::data::sprite::Sprite;
use crate::data::enemy::EnemyData;

mod operations;
use crate::operations::moving::move_straight;
use crate::operations::animating::animate;
use crate::operations::drawing::draw_sprite;

mod utils;
mod constants;

fn load_sprites() -> Result<Vec<Sprite>, io::Error> {
  let mut sprites = Vec::<Sprite>::new();
  let mut path = std::env::current_exe()?;
  path.push(sprite_folder_path());

  let dir = fs::read_dir(path)?;
  for entry in dir {
    let u_entry = entry?;
    if u_entry.metadata()?.is_file() && u_entry.file_name() != "player.txt" {
      let sprite = Sprite::load(&u_entry.path())?;
      sprites.push(sprite);
    }
  }

  return Ok(sprites);
}

fn load_player_sprite() -> Result<Sprite, io::Error> {
  let mut path = std::env::current_exe()?;
  path.push(sprite_folder_path());

  let dir = fs::read_dir(path)?;
  for entry in dir {
    let u_entry = entry?;
    if u_entry.metadata()?.is_file() && u_entry.file_name() == "player.txt" {
      return Ok(Sprite::load(&u_entry.path())?);
    }
  }

  return Err(Error::new(ErrorKind::Other, "Could not find file"));
}

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();
  let start = Instant::now();

  // Initialize terminal.
  enable_raw_mode()?;
  let (cols, rows) = size()?;
  let game_size : Vector2D<u16> = Vector2D::new(120, 32);
  execute!(stdout, SetSize(game_size.x, game_size.y))?;

  let player_sprite = load_player_sprite()?;
  let mut player_pos: Vector2D<f32> = Vector2D::new(0.0, game_size.y as f32 * 0.5 - 4.0);
  let mut player_frame: f32 = 0.0;
  let mut bullets = Vec::<Vector2D<f32>>::new();
  let mut cooldown = 0.0;

  let sprites = load_sprites()?;
  let mut enemies = Vec::<EnemyData>::new();
  let mut delta = 0.0;
  let mut health:usize = 50;

  'game_loop: loop {
    let frame_start = Instant::now();

    //1. Handle user input
    while poll(Duration::from_secs(0)).is_ok_and(|x| x == true) {
      let read_event = read()?;
      if let crossterm::event::Event::Key(keystroke) = read_event {
        if matches!(keystroke.code, KeyCode::Esc) || (matches!(keystroke.code, KeyCode::Char('c')) && matches!(keystroke.modifiers, KeyModifiers::CONTROL)) {
            break 'game_loop;
        }
        else if matches!(keystroke.code, KeyCode::Char('w')) {
          player_pos.y -= 1.0;
          player_pos.y = f32::max(player_pos.y, 0.0);
        }
        else if matches!(keystroke.code, KeyCode::Char('s')) {
          player_pos.y += 1.0;
          player_pos.y = f32::min(player_pos.y, game_size.y as f32);
        }
        else if matches!(keystroke.code, KeyCode::Char(' ')) && cooldown < 0.0 && bullets.len() < MAX_BULLET_COUNT {
          bullets.push(Vector2D::new(8.0, player_pos.y + 5.0));
          cooldown = 0.1;
        }
      }
    }

    //2. Update state
    cooldown -= delta;

    if enemies.len() < 2 && enemies.len() < MAX_ENEMY_COUNT {
      let assigned_sprite = rand_range(sprites.len());
      let assigned_sprite_height = sprites[assigned_sprite].max_height;
      enemies.push(EnemyData::new(start.elapsed().as_secs(), &game_size, assigned_sprite, assigned_sprite_height));
    }

    animate(&mut enemies, delta);
    move_straight(&mut enemies, delta);

    for enemy in enemies.iter_mut() {
      if enemy.translation.x < PLAYER_SIZE {
        let updated_health = health.checked_sub(enemy.damage);
        
        if updated_health.is_some() {
          health = updated_health.unwrap();
        }
        else {
          break 'game_loop;
        }
      }
    }

    for bullet in bullets.iter_mut() {
      bullet.x += delta * 50.0;
    }
    remove_bullets_off_screen(&mut bullets, game_size.x - 1);
    remove_enemy_bullets_under_collision(&mut enemies, &mut bullets, &sprites);
    
    //3, Draw state to screen
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    for enemy in enemies.iter() {
      let sprite = &sprites[enemy.texture_index];
      draw_sprite(sprite, enemy.current_frame as usize, &enemy.translation, &stdout, &game_size);
    }
    for bullet in bullets.iter() {
      let _ = stdout.queue(cursor::MoveTo(bullet.x as u16, bullet.y as u16));
      let _ = stdout.queue(style::Print('-'));
    }
    draw_sprite(&player_sprite, player_frame as usize, &player_pos, &stdout, &game_size);
    player_frame += delta * 10.0;
    let _ = stdout.queue(cursor::MoveTo(game_size.x - 10, game_size.y));
    let _ = stdout.queue(style::Print(delta));
    stdout.flush()?;

    delta = frame_start.elapsed().as_secs_f32();
  }

  //4. Exit/Cleanup
  execute!(io::stdout(), SetSize(cols, rows))?;
  Ok(())
}