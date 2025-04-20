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
pub  struct TileMap{
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
    
    pub fn get(&self,posx: usize, posy: usize) -> Option< &TileInfo>
    {
           return self.tiles.get(posy)?.get(posx);
    }


    pub fn get_mut(&mut self,posx: usize, posy: usize) -> Option<&mut TileInfo>
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
                }
                Err(e) => {
                    println!("OOB apply flow 2");

                }
            }
        }
        Err(e) => {
            println!("OOB apply flow");
        }
    };
}


pub fn calculate_flow(terrain:  &Terrain, source:  &TileInfo, destination:  &TileInfo) -> f64
{
    let h_diff = match terrain.height_at(source.coords.0 as i64, source.coords.1 as i64){
        Ok(h_src) => {
            match terrain.height_at(destination.coords.0 as i64, destination.coords.1 as i64) {
                Ok(h_dst) => {
                    let h_diff = h_src - h_dst;
                    let h_and_wter_h_diff = source.water_amount + h_src - (destination.water_amount + h_dst);
                    
                    let src_h_total = h_src + source.water_amount;
                    let dst_h_todal = h_dst+destination.water_amount;

                    if src_h_total < dst_h_todal{return 0.0;}
                    
                    let avg = (src_h_total + dst_h_todal)/2.0;

                    return src_h_total-avg;

                    



                }
                Err(e) => {
                    println!("OOB apply flow 2");
                    return  0.0;
                }
            }
        }
        Err(e) => {
            println!("OOB apply flow");
            return 0.0;
        }
    };
    return 0.0;
}


pub fn bfd_step(terrain: &mut Terrain, source: &mut TileMap, destination: &mut TileMap)
{
    let mut q:  VecDeque<(usize,usize)> = VecDeque::new(); 
    q.push_back((
        rand::rng().random_range(0..source.size_x) as usize, 
        rand::rng().random_range(0..source.size_y)as usize) );
    
    let mut visited :Vec<Vec<bool>> =vec![vec![false;source.size_x];source.size_y];

    while  !q.is_empty() {
        let posx = q.front().unwrap().0;
        let posy= q.front().unwrap().1;
        q.pop_front();
        if (visited[posx][posy]==true)
        {
            continue;
        }

        visited[posx][posy] = true;
        

        let mut flow_matrix: Vec<Vec<f64>> = vec![vec![0.0;3];3];
        let mut sum =0.0;

        let base = source.get(posx ,posy).unwrap(); // middle of the matrix
        for dx in (0..3){
            for dy in (0..3){
                if (dx != 0 && dy != 0){
                    match source.get(posx+dx-1, posy+dy-1) {
                        None=>{}
                        Some(H)=>{
                            // apply_flow(terrain,base, H);
                            let flow =  calculate_flow(terrain, base,H);
                            flow_matrix[dx][dy] = flow;
                            sum += flow;
                            if (visited[posx+dx-1][posy+dy-1]==false)
                            {
                                q.push_back((posx+dx-1,posy+dy-1));
                            }
                        }
                    }
                }
            }
        }

        let water_avaliable_to_flow = (base.water_amount )*0.3; /// ja to potem gdzieś wyrzuce jako parametr...... serio
        
        // take totaly random ammount of rock 
        let height_at_pos = terrain.height_at(posx as i64, posy as i64).unwrap() * 0.5;
        let mut  sediment_eroded_this_tick = base.water_amount*0.1;
        if sediment_eroded_this_tick > height_at_pos
        {
            sediment_eroded_this_tick = height_at_pos;
        }

        (*terrain.height_at_mut(posx as i64, posy as i64).unwrap())-=sediment_eroded_this_tick;
        let sediment_avaliable_to_flow = (base.sediment_amount + sediment_eroded_this_tick)*0.3;


        let flow_multiplier = water_avaliable_to_flow/sum;
        let sediment_multiplier = sediment_avaliable_to_flow/sum;
        for dx in (0..3){
            for dy in (0..3){
                if (dx != 0 && dy != 0){
                    match destination.get_mut(posx+dx-1, posy+dy-1) {
                        None=>{}
                        Some(H)=>{
                            (*H).water_amount += flow_matrix[dx][dy] * flow_multiplier;
                            (*H).sediment_amount += flow_matrix[dx][dy]*sediment_multiplier;

                        }
                    }
                }
            }
        }
    }
}