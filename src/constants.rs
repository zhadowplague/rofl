pub const MAX_ENEMY_COUNT : usize = 10;
pub const MAX_BULLET_COUNT : usize = 20;
pub const PLAYER_SIZE : f32 = 30.0;

#[cfg(not(debug_assertions))]
pub const SPRITE_FOLDER : &str = "sprites";
#[cfg(debug_assertions)]
pub const SPRITE_FOLDER_DEBUG : &str = "..\\..\\..\\sprites";

#[cfg(debug_assertions)]
pub fn sprite_folder_path() -> &'static str{
  return SPRITE_FOLDER_DEBUG
}

#[cfg(not(debug_assertions))]
pub fn sprite_folder_path() -> &'static str{
  return SPRITE_FOLDER;
}