use rand::prelude::*;
#[path = "../map.rs"] mod map;


pub fn basic_drop(map: &mut Map, drop_amnt: i32, drop_life: i32)
{
    let (h, w) = map.dimensions();
    
    let mut ret = map.height.clone();
    

    for iter in 0..drop_amnt{
        let pos_x: i64 = rand::gen_range(0,w);
        let pos_y: i64 = rand::gen_range(0,h);
        for l in 0..drop_life{
            let mut min_x = f64::MAX;
            let mut min_y = f64::MAX;
            let mut mini = f64::MAX;


            let mut tx;
            let mut ty;

            tx = pos_x-1;
            ty = pos_y-1;
            let surround = [(pos_x, pos_y-1), (pos_x,pos_y+1), (pos_x-1, pos_y),(pos_x+1,pos_y)];
            for (tx,ty) in surround.iter() {
                match map.height_at(tx, ty){
                    Ok(v) => {
                        if (mini > v) {
                            mini = v;
                            min_x = tx;
                            min_y = ty;
                        }
                    }
                    Err(e) => {

                    }
                }
            }

            
        }
    }

}


fn get(x: i64,y:i64,max_x: i64,max_y: i64, map: &Vec<Vec<f64>> ){
    if (x<0){return i64::MAX;}
    if (x>max_x){return i64::MAX;}
    if (y<0){return i64::MAX;}
    if (y>max_y){return i64::MAX;}
    return map[y][x];
}
