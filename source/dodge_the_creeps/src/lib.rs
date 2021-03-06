use gdnative::prelude::*;
mod extensions;
mod game;
mod hud;
mod mob;
mod player;

/// Expose all Rust code for Godot.
pub fn init(handle: InitHandle) {
  handle.add_class::<game::GameHandler>();
  handle.add_class::<hud::HUD>();
  handle.add_class::<mob::Mob>();
  handle.add_class::<player::Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
