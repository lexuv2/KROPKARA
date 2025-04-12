use godot::prelude::*;
use godot::classes::Control;
use godot::classes::IControl;

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

