use std::path;

use godot::prelude::*;
use godot::classes::Control;
use godot::classes::IControl;
use image_generator::array_to_image;
use image_generator::get_random_2d_noise;

#[path = "image_generator.rs"] mod image_generator;

#[derive(GodotClass)]
#[class(base=Control)]
struct Caller{
    test: f64,
    base: Base<Control>


}
#[godot_api]
impl IControl for Caller{
    fn init(base: Base<Control>) -> Self
    {
        godot_print!("Hello, world!");

        Self{
            test: 10.0,
            base,
        }

    }
}
#[godot_api]
impl Caller {
    #[func]
    fn on_click(){
        let noise = get_random_2d_noise(256,256);
        array_to_image(noise);
    }
}
