use rand::prelude::*;
#[path = "../map.rs"] mod map;
use map::Map;


pub fn basic_drop(map: &mut Map, drop_amnt: i32, drop_life: i32)
{
    let (h, w) = map.dimensions();
    
    let mut ret = map.height.clone();
    

    for iter in 0..drop_amnt{
        let pos_x: i64 = rand::thread_rng().gen_range(0..(w as i64));
        let pos_y: i64 = rand::thread_rng().gen_range(0..(h as i64));
        for l in 0..drop_life{
            let mut min_x:i64 = 0;
            let mut min_y:i64 = 0;
            let mut mini = f64::MAX;


            let mut tx: i64;
            let mut ty: i64;

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

            
        }
    }

}


