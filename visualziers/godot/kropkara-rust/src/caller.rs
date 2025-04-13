use std::path;

use basic_drop::basic_drop;
use godot::meta::AsArg;
use godot::meta::ParamType;
use godot::prelude::*;
use godot::classes::Control;
use godot::classes::IControl;
use godot::classes::Texture2D;
use godot::classes::*;
use godot::classes::Image;
use crate::terrain::*;
use crate::image_generator::*;
#[path = "droppers/basic_drop.rs"] mod basic_drop;

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
    fn on_click(&self){
        let terrain: Terrain = Terrain::new_noise(256, 256, 1.1, 2);
        array_to_image(terrain.height);
    }
    #[func]
    fn perlin(&self, xdmin: i64, ydmin: i64, sp:f64, iters: i64) -> Array<PackedFloat64Array>{
        let terrain: Terrain = Terrain::new_noise(xdmin, ydmin, sp, iters);
        return  Caller::get_godot_vec_array_from_terrain(terrain);
    }

    #[func]
    fn godot_basic_drop(drop_amnt: i64,drop_life: i64,erosion: f64)
    {
        let mut terrain: Terrain = Terrain::new_noise(512, 512, 1.0, 1);
        basic_drop(&mut terrain, drop_amnt, drop_life,erosion);
        array_to_image(terrain.height);



    }

    #[func]
    fn generate_image_from_array(arr:Array<PackedFloat64Array>)
    {
        let rust_arr = Caller::get_vec_arr_from_godot(arr);
        array_to_image(rust_arr);
        
    }

    fn get_godot_vec_array_from_terrain(terrain: Terrain) -> Array<PackedFloat64Array>
    {
        let mut ret: Array<PackedFloat64Array> = Array::new();
        let heightterrain: Vec<Vec<f64>> = terrain.height;

        for x in heightterrain.iter(){
            let arr = PackedFloat64Array::from(x.clone());
            ret.push(arr.owned_to_arg());
        }
        return  ret;
    }


    fn get_vec_arr_from_godot(arr:  Array<PackedFloat64Array>) -> Vec<Vec<f64>>
    {
        let mut ret: Vec<Vec<f64>>= vec![];
        for x in arr.iter_shared(){
            let single: Vec<f64> = x.to_vec();
            ret.push(single);
        }
        return ret

        
    }
}
