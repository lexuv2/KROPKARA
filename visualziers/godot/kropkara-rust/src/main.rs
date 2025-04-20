mod terrain;
mod image_generator;
use crate::terrain::*;
use crate::image_generator::*;


#[path = "droppers/basic_drop.rs"] mod basic_drop;
#[path = "droppers/bfd_drop.rs"] mod bfd_drop;


mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}
pub fn main()
{
    let x_size = 512 as usize;    
    let y_size = 512 as usize;   
    let amnt = 1000;
    let mut terrain =  Terrain::new_noise(512, 512, 1.0, 1);


    
    let mut first_map= bfd_drop::TileMap::create_initial(x_size ,y_size );
    let mut second_map = bfd_drop::TileMap::create_initial(x_size, y_size );
    for i in 0..amnt{
        if i%2 == 0
        {
            bfd_drop::bfd_step(&mut terrain, &mut first_map, &mut second_map);
        }
        else
        {
            bfd_drop::bfd_step(&mut terrain, &mut second_map, &mut first_map);
        }
        array_to_image(&terrain.height,"TEST");
        simple_user_input::get_input("press any key to continue");
    }
}