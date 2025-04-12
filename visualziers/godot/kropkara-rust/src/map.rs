use rand::prelude::*;
use godot::prelude::*;

#[derive(Debug)]
pub struct Map {
    pub height: Vec<Vec<f64>>,
    softness: Vec<Vec<f64>>,
    absorption: Vec<Vec<f64>>,
    dropmap: Vec<Vec<f64>>,
    speedmap: Vec<Vec<f64>>,
    speed_preservation: f64,
    x_size: i64,
    y_size: i64,
}

impl Map{
    fn new(x: i64, y:i64, sp: f64) -> Map {
        let mut m = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut s = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut a = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut dm = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut sm = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        Map {
            height: m,
            softness: s,
            absorption: a,
            dropmap: dm,
            speedmap: sm,
            speed_preservation: sp,
            x_size: x,
            y_size: y
        }
    }

    fn height_at(&self, x: i64, y:i64) -> Result<f64, &str> {
        match self.height.get(x as usize) {
            Some(out) => {
                match out.get(y as usize) {
                    Some(v) => {
                        Ok(v.clone())
                    }
                    None => {Err("OOB")}
                }
            }
            None => {Err("OOB")}
        }
    }

    fn check_oob(&self, x: i64, y: i64) -> bool {
        (x >= 0) && (x < self.x_size) && (y >= 0) && (y < self.y_size)
    }

    /*
    // SYF KTORY JESZCZENIE NIE DZIALA
    fn save(&self, filename: &str) {
        let hmap_name: String = format!("{}_hmap", filename);
        let smap_name: String = format!("{}_softmap", filename);
        let amap_name: String = format!("{}_absorptionmap", filename);
        let dmmap_name: String = format!("{}_dropmap", filename);
        let smmap_name: String = format!("{}_speedmap", filename);
        let spmap_name: String = format!("{}_map", filename);
    }
    */

    pub fn new_noise(x: i64, y: i64, sp: f64, iters: i64) -> Map {
        let mut m = Map::new(x,y,sp);
        let ii:i32 = rand::thread_rng().gen_range(0..255);
        let jj:i32 = rand::thread_rng().gen_range(0..255);
        let mut disp:f64 = 1.0;

        for iter in 0..iters {
            disp = disp * 2.0;
            for (xi,vi) in m.height.iter_mut().enumerate() {
                for (yj,vj) in vi.iter_mut().enumerate() {
                    let using_x = (xi as f64)/(x as f64)*4.0 + (ii as f64) * disp;
                    let using_y = (yj as f64)/(x as f64)*4.0 + (jj as f64) * disp;
                    let q = 100.0*perlin(using_x, using_y) / disp;
                    *vj = *vj + q;
                }
            }
        }
        m
    }
}

// SYF DO PRZENIESIENIA KIEDYŚ GDZIEŚ INDZIEJ

fn interpolate (a0:f64, a1:f64, w:f64) -> f64 {
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

fn random_gradient (ix:i64, iy:i64) -> [f64; 2] {
    let w:u64 = 32;
    let s:u64 = 16;
    let mut a:u64 = ix as u64;
    let mut b:u64 = iy as u64;
    
    a = a.overflowing_mul(3284157443).0;
    b ^= a << s | a >> w-s;
    b = b.overflowing_mul(1911520717).0;
    a ^= b << s | b >> w-s;
    a = a.overflowing_mul(2048419325).0;
    let randi = (a as f64) * (3.14159265 / 9223372036854775807.5 );
    let v = [randi.cos(), randi.sin()];
    v
}

fn dot_grid_gradient(ix:i64, iy:i64, x:f64, y:f64) -> f64 {
    let gradient = random_gradient(ix, iy);
    let dx = x - (ix as f64);
    let dy = y - (iy as f64);
    dx*gradient[0] + dy*gradient[1]
}

fn perlin(x:f64, y:f64) -> f64 {
    let x0 = x.floor() as i64;
    let x1 = x0 + 1;
    let y0 = y.floor() as i64;
    let y1 = y0 + 1;
    let sx = x - (x0 as f64);
    let sy = y - (y0 as f64);

    let n0 = dot_grid_gradient(x0, y0, x, y);
    let n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);
    let n0 = dot_grid_gradient(x0, y1, x, y);
    let n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);
    
    interpolate(ix0, ix1, sy)
}
