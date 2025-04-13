use rand::prelude::*;

pub fn basic_drop(map: Vec<Vec<f64>>, drop_amnt: i32, drop_life: i32)
{
    let h = map.len();
    let w = map[0].len();

    
    let mut ret = map.clone();
    

    for iter in 0..drop_amnt{
        let pos_x: i64 = rand::gen_range(0,w);
        let pos_y: i64 = rand::gen_range(0,h);
        for l in 0.drop_life{
            let mut min_x = i64::MAX;
            let mut min_y = i64::MAX;
            let mut mini = i64::MAX;


            let mut tx;
            let mut ty;

            tx = pos_x-1;
            ty - pos_y-1;
            if (get(tx,ty,w,h,map) < mini ){min_x = tx;min_y=ty; mini = get(tx,ty,w,h,map);}

            tx = pos_x;
            ty - pos_y-1;
            if (get(tx,ty,w,h,map) < mini ){min_x = tx;min_y=ty; mini = get(tx,ty,w,h,map);}

            tx = pos_x+1;
            ty - pos_y-1;
            if (get(tx,ty,w,h,map) < mini ){min_x = tx;min_y=ty; mini = get(tx,ty,w,h,map);}

            tx = pos_x+1;
            ty - pos_y+1;
            if (get(tx,ty,w,h,map) < mini ){min_x = tx;min_y=ty; mini = get(tx,ty,w,h,map);}

            
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