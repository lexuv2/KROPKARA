mod caller;
mod terrain;
mod image_generator;
use godot::prelude::*;


struct KropkaraRust;

#[gdextension]
unsafe impl ExtensionLibrary for KropkaraRust {}


// cargo build
// godot4 main_window.tscn
