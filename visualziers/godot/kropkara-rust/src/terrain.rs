use rand::prelude::*;
use godot::prelude::*;
#[path = "perlin.rs"] mod perlin;

#[derive(Debug)]
pub struct Terrain {
    pub height: Vec<Vec<f64>>,
    softness: Vec<Vec<f64>>,
    absorption: Vec<Vec<f64>>,
    dropterrain: Vec<Vec<f64>>,
    speedterrain: Vec<Vec<f64>>,
    speed_preservation: f64,
    x_size: i64,
    y_size: i64,
}

impl Terrain{
    pub fn new(x: i64, y:i64, sp: f64) -> Terrain {
        let mut m = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut s = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut a = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut dm = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        let mut sm = vec![vec![0.0; (x+1) as usize] ; (y+1) as usize];
        Terrain {
            height: m,
            softness: s,
            absorption: a,
            dropterrain: dm,
            speedterrain: sm,
            speed_preservation: sp,
            x_size: x,
            y_size: y
        }
    }

    pub fn normalize(terrain: &mut Vec<Vec<f64>>, min_val: f64, max_val: f64)
    {

    }

    pub fn height_at(&self, x: i64, y:i64) -> Result<f64, &str> {
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

    pub fn check_oob(&self, x: i64, y: i64) -> bool {
        (x >= 0) && (x < self.x_size) && (y >= 0) && (y < self.y_size)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.height.len(), self.height[0].len())
    }

    /*
    // SYF KTORY JESZCZENIE NIE DZIALA
    fn save(&self, filename: &str) {
        let hterrain_name: String = format!("{}_hterrain", filename);
        let sterrain_name: String = format!("{}_softterrain", filename);
        let aterrain_name: String = format!("{}_absorptionterrain", filename);
        let dmterrain_name: String = format!("{}_dropterrain", filename);
        let smterrain_name: String = format!("{}_speedterrain", filename);
        let spterrain_name: String = format!("{}_terrain", filename);
    }
    */

    pub fn new_noise(x: i64, y: i64, sp: f64, iters: i64) -> Terrain {
        let mut m = Terrain::new(x,y,sp);
        let ii:i32 = rand::thread_rng().gen_range(0..255);
        let jj:i32 = rand::thread_rng().gen_range(0..255);
        let mut disp:f64 = 1.0;

        for iter in 0..iters {
            disp = disp * 2.0;
            for (xi,vi) in m.height.iter_mut().enumerate() {
                for (yj,vj) in vi.iter_mut().enumerate() {
                    let using_x = (xi as f64)/(x as f64)*4.0 + (ii as f64) * disp;
                    let using_y = (yj as f64)/(x as f64)*4.0 + (jj as f64) * disp;
                    let q = 100.0*perlin::perlin(using_x, using_y) / disp;
                    *vj = *vj + q;
                }
            }
        }
        m
    }
}

// SYF DO PRZENIESIENIA KIEDYŚ GDZIEŚ INDZIEJ
