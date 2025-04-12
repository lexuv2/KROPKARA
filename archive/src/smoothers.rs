use crate::XMAX;
use crate::YMAX;
use crate::check_coords;

pub fn smooth4 (map: &Vec<Vec<f64>>, range:u64, selfbias:f64) -> Vec<Vec<f64>> {   //nxn no bias inc border

    let mut map2 = map.clone();
    
    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            map2[i as usize][j as usize] = {
                let mut q:f64 = 0.0;
                let mut x:f64 = 0.0;
                for ii in ((i as i64)-(range as i64))..((i+1+range) as i64){
                    for jj in ((j as i64)-(range as i64))..((j+1+range) as i64){
                        if (ii>=0) && (ii<=(XMAX as i64)) && (jj>=0) && (jj<=(YMAX as i64)) {
                            q = q + 1.0;
                            x += map[ii as usize][jj as usize];
                        }
                    }
                }
                x += map[i as usize][j as usize] * (selfbias - 1.0);
                x/(q + selfbias - 1.0)
            }
        }
    }

    map2
}


pub fn smoothplace (map: &Vec<Vec<f64>>, range:i64, selfbias:f64, target: &mut Vec<Vec<f64>>,x:i64, y:i64) {
    let mut q:f64 = 0.0;
    let mut a:f64 = 0.0;
    for i in (x - range)..(x+1+range) {
        for j in (y - range)..(y+1+range) {
            if check_coords(i, j) && ((x-i)*(x-i) + (y-j)*(y-j) <= range*range) {
                q = q + 1.0;
                a += map[i as usize][j as usize];
            }
        }
    }

    a += map[x as usize][y as usize] * (selfbias - 1.0);
    a = a/(q + selfbias - 1.0);
    target[x as usize][y as usize] = a;
}

pub fn smootharea (map: &mut Vec<Vec<f64>>, range:i64, selfbias:f64, x:i64, y:i64) {
    let mut map2 = map.clone();

    for i in (x - range)..(x+1+range) {
        for j in (y - range)..(y+1+range) {
            if check_coords(i, j) && ((x-i)*(x-i) + (y-j)*(y-j) <= range*range) {
                smoothplace(map, range, selfbias, &mut map2, x, y);
            }
        }
    }
    for i in (x - range)..(x+1+range) {
        for j in (y - range)..(y+1+range) {
            if check_coords(i, j) && ((x-i)*(x-i) + (y-j)*(y-j) <= range*range) {
                map[i as usize][j as usize] = map2[i as usize][j as usize];
            }
        }
    }
}

pub fn smootharea2 (map: &mut Vec<Vec<f64>>, range:i64, selfbias:f64, x:i64, y:i64) {

    let mut map2 = vec![ vec![-1.0 as f64; (range*2+1) as usize] ; (range*2+1) as usize];   //map2[range*2][range*2] === map[x][y]

    for i in (-range)..(range+1) {
        for j in (-range)..(range+1) {
            if (i*i + j*j <= range*range) && check_coords(x+i, y+j) {   //valid offsets
                let mut a: f64 = 0.0;
                let mut q: f64 = 0.0;
                for ii in (-range)..(range+1) {
                    for jj in (-range)..(range+1) {
                        if (ii*ii + jj*jj <= range*range) && check_coords(x+i+ii, y+j+jj) {
                            q = q + 1.0;
                            a += map[(x+i+ii) as usize][(y+j+jj) as usize];
                        }
                    }
                }
                a += map[(x+i) as usize][(y+j) as usize] * (selfbias - 1.0);
                a = a / (q + selfbias - 1.0);
                map2[(i+range) as usize][(j+range) as usize] = a;
            }
        }
    }


    //println!("smooth {} , {}", x , y);

    for i in (-range)..(range+1) {
        for j in (-range)..(range+1) {
            if check_coords(x+i, y+j) == false {
                continue;
            }
            if map2[(i+range) as usize][(j+range) as usize] < 0.0 {
                continue;
            }
    //        println!("{} {} {} {}", x+i, y+j, i+range, j+range);
            map[(x+i) as usize][(y+j) as usize] = map2[(i+range) as usize][(j+range) as usize];
        }
    }
}

pub fn upmap (map: &mut Vec<Vec<f64>>, h:f64){
    for i in 0..(XMAX + 1){
        for j in 0..(YMAX + 1){
            map[i as usize][j as usize] += h;
        }
    }
}

pub fn radicalize (map: &Vec<Vec<f64>>, factor: f64) -> Vec<Vec<f64>>{

    let mut map2 = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];

    //let mut inc = 0.0;

    for i in 0..(XMAX+1){
        for j in 0..(YMAX+1){
            map2[i as usize][j as usize] = map[i as usize][j as usize] * factor;
    //        inc += map2[i as usize][j as usize] - map[i as usize][j as usize];
        }
    }
    /*
    inc /= ((XMAX+1)*(YMAX+1)) as f64;

    for i in 0..(XMAX+1){
        for j in 0..(YMAX+1){
            map2[i as usize][j as usize] = (0.0 as f64).max( map2[i as usize][j as usize] - inc);   //flat 100
            
            //map2[i as usize][j as usize] = ZMAX.min(map2[i as usize][j as usize]);
        }
    }
    */

    map2
}

pub fn pushdown (map: &mut Vec<Vec<f64>>) {
    let mut mini:f64 = f64::MAX;
    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            mini = mini.min(map[i as usize][j as usize]);
        }
    }
    
    mini -= 100.0;

    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            map[i as usize][j as usize] -= mini;
        }
    }
}
