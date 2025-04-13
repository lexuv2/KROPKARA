use rand::prelude::*;
use crate::map::*;


pub fn basic_drop(map: &mut Map, drop_amnt: i64, drop_life: i64,erosion_factor: f64)
{
    let (h, w) = map.dimensions();
    
    let mut ret = map.height.clone();
    

    for iter in 0..drop_amnt{
        let mut pos_x: i64 = rand::thread_rng().gen_range(0..(w as i64));
        let mut pos_y: i64 = rand::thread_rng().gen_range(0..(h as i64));
        for l in 0..drop_life{
            let mut min_x:i64 = 0;
            let mut min_y:i64 = 0;
            let mut mini = f64::MAX;


            let mut tx: i64;
            let mut ty: i64;

            map.height[pos_y as usize][pos_x as usize] -=erosion_factor;

            let surround = [(pos_x, pos_y-1), (pos_x,pos_y+1), (pos_x-1, pos_y),(pos_x+1,pos_y)];
            for (tx,ty) in surround.iter() {
                match map.height_at(*tx, *ty){
                    Ok(v) => {
                        if (mini > v) {
                            mini = v;
                            min_x = *tx;
                            min_y = *ty;
                        }
                    }
                    Err(e) => {

                    }
                }
            }
            
            pos_x = min_x;
            pos_y = min_y;

            
        }
    }

}


