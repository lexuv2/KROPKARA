// generate images from 2d array

use image::{ImageError, RgbImage};
use rand::prelude::*;

use godot::prelude::*;

pub fn array_to_image(arr: Vec<Vec<f64>>) -> RgbImage
{
    let h: usize = arr.len();
    let w: usize = arr[0].len();
    println!("{h}");
    println!("{w}");
    let mut buf: Vec<u8> = vec![];


    let mut min: f64 = std::f64::MAX;
    let mut max:f64 = std::f64::MIN;

    for i in (0..h){
        for j in (0..w)
        {
            let val = arr[i as usize][j as usize];
            if (min > val){ min=val;}
            if (max < val){max = val;}


            // min = std::cmp::min(arr[i as usize][j as usize], min);
            // max = std::cmp::max(arr[i as usize][j as usize], max);
        }
    }
    
    for i in (0..h){
        for j in (0..w)
        {
           
            
            let val = arr[i as usize][j as usize];
            let norm: f64 = (val - min)/(max-min) *255.0;
            let norm_u8: u8 = norm as u8;

            buf.push(norm_u8);
            buf.push(norm_u8);
            buf.push(norm_u8);
        }
    }


    let buf_len = buf.len();
    godot_print!("{buf_len}");
    godot_print!("{h}");
    godot_print!("{w}");
    let img_buf = RgbImage::from_raw(w as u32, h as u32, buf);


    let img = img_buf.expect("{img_buf.Err}");
    img.save("TEST.png").unwrap();
    return  img;
}


pub  fn get_random_2d_noise(h: i32,w:i32) -> Vec<Vec<f64>>
{
    let mut rng = rand::rng();
    let mut base: Vec<Vec<f64>>= vec![];
    for i in (0..h){
        base.push(vec![]);
        for j in (0..w)
        {
            base[i as usize].push(rng.random::<f64>());
        }
    }
    
    return base;
    
}