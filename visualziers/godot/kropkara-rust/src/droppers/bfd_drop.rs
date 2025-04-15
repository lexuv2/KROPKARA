use rand::prelude::*;
use crate::terrain::*;
use std::collections::VecDeque;

#[derive(Clone)]
#[derive(Debug)]
struct TileInfo{
	pub water_amount: f64,
	pub sediment_amount: f64,
	pub speed: (f64, f64),
    pub coords: (usize, usize),
}

impl TileInfo {
	pub fn capacity(&self) -> f64 {
		let (x,y) = self.speed;
		return x*x+y*y
	}
}

#[derive(Clone)]
#[derive(Debug)]
struct TileMap{
    pub tiles: Vec<Vec<TileInfo>>,
    pub size_x: usize,
    pub size_y: usize,
}

impl TileMap{
    pub fn create_initial(size_x: usize, size_y: usize) -> TileMap{
        let empty_tile = TileInfo{
            sediment_amount: 0.0,
            water_amount: 1.0,
            speed: (0.0,0.0),
            coords: (0,0),
        };
        let mut ret = vec![vec![empty_tile;size_x];size_y];

        for x in (0..size_x){
            for y in (0..size_y){
                ret[x][y].water_amount = rand::rng().random_range(0.0..1.0);
                ret[x][y].coords = (x,y);
            }
        }
        return TileMap { tiles: ret, size_x: size_x, size_y: size_y }
    }
    
    pub fn get(&mut self,posx: usize, posy: usize) -> Option<&mut TileInfo>
    {
           return self.tiles.get_mut(posy)?.get_mut(posx);
    }
}


pub fn apply_flow(terrain: &mut Terrain, source: &mut TileInfo, destination: &mut TileInfo)
{
    let h_diff = match terrain.height_at(source.coords.0 as i64, source.coords.1 as i64){
        Ok(h_src) => {
            match terrain.height_at(destination.coords.0 as i64, destination.coords.1 as i64) {
                Ok(h_dst) => {
                    let h_diff = h_src - h_dst;
                    let h_and_wter_h_diff = source.water_amount + h_src - (destination.water_amount + h_dst);
                    
                        /*
                        
                        
__  __    _    ____ ___ _____ ____   __________ 
|  \/  |  / \  / ___|_ _| ____|  _ \ |__  / ____|
| |\/| | / _ \| |    | ||  _| | |_) |  / /|  _|  
| |  | |/ ___ \ |___ | || |___|  _ <  / /_| |___ 
|_|  |_/_/   \_\____|___|_____|_| \_\/____|_____|



 __  __    _    ____ ___ _____ ____   __________ 
|  \/  |  / \  / ___|_ _| ____|  _ \ |__  / ____|
| |\/| | / _ \| |    | ||  _| | |_) |  / /|  _|  
| |  | |/ ___ \ |___ | || |___|  _ <  / /_| |___ 
|_|  |_/_/   \_\____|___|_____|_| \_\/____|_____|



 __  __    _    ____ ___ _____ ____   __________ 
|  \/  |  / \  / ___|_ _| ____|  _ \ |__  / ____|
| |\/| | / _ \| |    | ||  _| | |_) |  / /|  _|  
| |  | |/ ___ \ |___ | || |___|  _ <  / /_| |___ 
|_|  |_/_/   \_\____|___|_____|_| \_\/____|_____|


 __  __    _    ____ ___ _____ ____   __________ 
|  \/  |  / \  / ___|_ _| ____|  _ \ |__  / ____|
| |\/| | / _ \| |    | ||  _| | |_) |  / /|  _|  
| |  | |/ ___ \ |___ | || |___|  _ <  / /_| |___ 
|_|  |_/_/   \_\____|___|_____|_| \_\/____|_____|

                        
                        
                        
                        
                         */

                }
                Err(e) => {
                    println!("OOB apply flow 2");
                    0.0
                }
            }
        }
        Err(e) => {
            println!("OOB apply flow");
            0.0
        }
    };
}

pub fn bfd(terrain: &mut Terrain, source: &mut TileMap, destination: &mut TileMap)
{
    let mut q:  VecDeque<(usize,usize)> = VecDeque::new(); 
    q.push_back((
        rand::rng().random_range(0..source.size_x) as usize, 
        rand::rng().random_range(0..source.size_y)as usize) );
    
    let mut visited :Vec<Vec<bool>> =vec![vec![false;source.size_x];source.size_y];

    while  !q.is_empty() {
        let posx = q.front().unwrap().0;
        let posy= q.front().unwrap().1;
        let mut base = source.get(posx ,posy).unwrap();
        for dx in (0..3){
            for dy in (0..3){
                if (dx != 0 && dy != 0){
                    match source.get(posx+dx-1, posy+dy-1) {
                        None=>{}
                        Some(H)=>{
                            apply_flow(terrain,base, H);
                        }
                    }
                }
            }
        }
        
    }
}