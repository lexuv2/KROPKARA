
//-----------------------------------------------------------------


const XMAX:u64 = 299;
const YMAX:u64 = 299;
const ZMININI:f64 = 2000.0;
const ZMAX:f64 = 10000.0;

pub mod smoothers;
pub mod generators;

use crate::generators::*;
use crate::smoothers::*;

use rand::Rng;
use std::time;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use ansi_control::*;
//use std::mem;


fn main() {
    println!("Hello, world! ");

    /*println!("inport x");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("x input failed");
    let x:f64 = x.trim().parse().expect("x parse failed");

    println!("inport y");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("y input failed");
    let y:f64 = y.trim().parse().expect("y parse failed");

    println!("{}", perlin(x, y));*/

    
    //      _~^~^~_
    // \/ /  o o  \ \/
    //   '_   O   _'
    //   \ '-----' /

    
    let mut map = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize]; //pre smooth

    let mut map2 = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize]; //after smoothing



    //let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];  //heatmap for raindrops

    println!("-------------------------------------------------");


    let mapname = "basemap.txt";
    let map2name = "aftersmooth.txt";
    let map3name = "afterrain.txt";
    let hmapname = "whererain.txt";
    let leftmapname = "whereset.txt";
    let speedmapname = "wherespeeed.txt";
    let subheatmapname = "submap.txt";

    loop {

        println!("\n------------------------------------\nmode?");
        let mut mode = String::new();
        io::stdin().read_line(&mut mode).expect("mode input failed");

        mode = mode.to_lowercase();

        match mode.trim() {
            "p" | "P" => {

                /*
                println!("inport max height");
                let mut maxheight = String::new();
                io::stdin().read_line(&mut maxheight).expect("maxheight input failed");
                let maxheight:f64 = maxheight.trim().parse().expect("maxheight parse failed");

                println!("inport min height");
                let mut minheight = String::new();
                io::stdin().read_line(&mut minheight).expect("minheight input failed");
                let minheight:f64 = minheight.trim().parse().expect("minheight parse failed");
                */

                let ii:i32 = rand::thread_rng().gen_range(0..1000000);
                let jj:i32 = rand::thread_rng().gen_range(0..1000000);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        map2[i as usize][j as usize] = perlin_scaling_abs(perlin((i as f64)/(100.0) + (ii as f64), (j as f64)/(100.0) + (jj as f64))) * (ZMAX - ZMININI) + ZMININI;  //general
                    }
                }


                let mut o2:f64 = 1.0;
                for o in (-2)..(10 as i32) {
                    /*match (o < 0) {
                        true => o2 = 1.0 / 2_i64.pow((o*-1) as u32) as f64,
                        false => 
                    }*/
                    if o < 0 {

                        o2 = 1.0 / 2_i64.pow((o*-1) as u32) as f64;
                    }
                    else {
                        o2 = 2_i64.pow(o as u32) as f64;
                    }

                    for i in 0..(XMAX+1) {
                        for j in 0..(YMAX+1) {
                            map2[i as usize][j as usize] += perlin_scaling_abs(perlin((i as f64)/(100.0)*o2 + (ii as f64), (j as f64)/(100.0)*o2 + (jj as f64))) * (ZMAX - ZMININI)/o2 + ZMININI/o2;   //general
                        }
                    }
                }

                /*let ii:i32 = rand::thread_rng().gen_range(0..1000000);
                let jj:i32 = rand::thread_rng().gen_range(0..1000000);
                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        map2[i as usize][j as usize] += perlin_scaling_abs(perlin((i as f64)/(10.0) + (ii as f64), (j as f64)/(10.0) + (jj as f64))) * (ZMAX - ZMININI)/50.0 + ZMININI/50.0;  //local
                    }
                }*/

                

                pushdown(&mut map2);

                savemap(&map2, map2name);

            }
            "n" | "N" | "m" | "M" => {
            //"1" | "map" | "Map" | "new" | "New" | "newmap" | "m" | "M" => {  //regen map
                ini_map(&mut map);
                savemap(&map, &mapname);
            }
            "s" | "S" => {
            //"2" | "smo" | "Smo" | "smooth" | "Smooth" | "resmooth" | "s" | "S" => {  //new map2

                println!("inport smootthrough");
                let mut smootthrough = String::new();
                io::stdin().read_line(&mut smootthrough).expect("smooththrough input failed");
                let smootthrough:i64 = smootthrough.trim().parse().expect("smooththrough parse failed");

                println!("inport smoothing");
                let mut smoothing = String::new();
                io::stdin().read_line(&mut smoothing).expect("smoothing input failed");
                let smoothing:i64 = smoothing.trim().parse().expect("smoothing parse failed");

                println!("inport radicalize");
                let mut rad = String::new();
                io::stdin().read_line(&mut rad).expect("radicalize input failed");
                let rad:f64 = rad.trim().parse().expect("radicalize parse failed");

                println!("inport range");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:u64 = range.trim().parse().expect("range parse failed");

                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                map2 = map.clone();

                for i in 0..smootthrough {
                    for _j in 0..smoothing {
                        map2 = smooth4(&map2, range, selfbias);
                    }
                    map2 = radicalize(&map2, rad);
                    

                    if i == 0 {
                        println!("{} / {} ({:.2}%)", i+1, smootthrough, ( (i as f64) / (smootthrough as f64) * 100.0));
                    }
                    else{
                        println!("{}{}{}{} / {} ({:.2}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, smootthrough, ( (i as f64) / (smootthrough as f64) * 100.0));
                    }
                }
                println!("{}{}{}new map aquired",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));
                upmap(&mut map2, 100.0);
                pushdown(&mut map2);
                
                savemap(&map2, &map2name);
            }
            /*//drop1 code
            '3' => {
//            "3" | "rain" | "Rain" | "r" | "R" | "drops" | "Drops" | "d" | "D" => {  //new map3

                let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];

                println!("inport rainfall");
                let mut rainfall = String::new();
                io::stdin().read_line(&mut rainfall).expect("rainfall input failed");
                let rainfall:i64 = rainfall.trim().parse().expect("rainfall parse failed");
                
                println!("inport ero");
                let mut ero = String::new();
                io::stdin().read_line(&mut ero).expect("ero input failed");
                let ero:f64 = ero.trim().parse().expect("ero parse failed");
        
                println!("inport material loss");
                let mut loss = String::new();
                io::stdin().read_line(&mut loss).expect("material loss input failed");
                let loss:f64 = loss.trim().parse().expect("material loss parse failed");
        
                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                println!("inport range");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:u64 = range.trim().parse().expect("range parse failed");

                let mut map3 = map2.clone();
                
        
                for i in 0..rainfall {
                    if (i != 0) && ( i % (((XMAX + 1) * (YMAX + 1)) as i64) == 0){
                        map3 = smooth4(&map3, range, selfbias);
                    }
                    drop1(&mut map3, ero, loss, &mut hmap);
                    
                    
                    if i == 0 {
                        println!("{} / {} ({:.2}%)", i+1, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    else{
                        println!("{}{}{}{} / {} ({:.2}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                }
                println!("{}{}{}", clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));
                savemap(&map3, &map3name);
                savemap(&hmap, &hmapname);

            }
            */
            /*  //drop2 code
            'r' | 'R' | 'd' | 'D' => {
                let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];

                let mut to = vec![ vec![0 as i64; (YMAX+1) as usize] ; (XMAX+1) as usize];

                let mut ts = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];

                let mut sm = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];


                println!("inport rainfall");
                let mut rainfall = String::new();
                io::stdin().read_line(&mut rainfall).expect("rainfall input failed");
                let rainfall:i64 = rainfall.trim().parse().expect("rainfall parse failed");
                
                println!("inport ero");
                let mut ero = String::new();
                io::stdin().read_line(&mut ero).expect("ero input failed");
                let ero:f64 = ero.trim().parse().expect("ero parse failed");
        
                println!("inport speeddump");
                let mut speeddump = String::new();
                io::stdin().read_line(&mut speeddump).expect("self bias input failed");
                let speeddump:f64 = speeddump.trim().parse().expect("self bias parse failed");
        
                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                println!("inport range");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:u64 = range.trim().parse().expect("range parse failed");

                println!("inport smooth inverval");
                let mut smoothterval = String::new();
                io::stdin().read_line(&mut smoothterval).expect("smoothterval input failed");
                let smoothterval:i64 = smoothterval.trim().parse().expect("smoothterval parse failed");
                
                
                

                let mut map3 = map2.clone();
                
        
                for i in 0..rainfall {
//                    println!("\n----------------");
                    if (i != 0) && ( i % smoothterval == 0){
                        map3 = smooth4(&map3, range, selfbias);
                        //println!("cha cha real smooth");
                    }


                    let xx:i64 = rand::thread_rng().gen_range(0..((XMAX + 1) as i64));
                    let yy:i64 = rand::thread_rng().gen_range(0..((YMAX + 1) as i64));

                    drop2(&mut map3, xx , yy , 1.0 , 8 as i64, 0.0, &xx, &yy, &mut hmap, &speeddump, &ero, 0, &mut to, &(i+1), 0.0, &mut ts, &mut sm);
                    
                    /*
                    if i == 0 {
                        println!("{} / {} ({:.2}%)", i+1, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    else{
                        println!("{}{}{}{} / {} ({:.2}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    */
                }

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        sm[i as usize][j as usize] /= hmap[i as usize][j as usize];
                    }
                }

                //println!("{}{}{}", clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));
                savemap(&map3, &map3name);
                savemap(&hmap, &hmapname);
                savemap(&ts, &leftmapname);
                savemap(&sm, &speedmapname);
                println!("{}", avmap(&mut map3));
            }
            */
            "d" | "D" => {
                let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut to = vec![ vec![0 as i64; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut ts = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut sm = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];

                println!("inport rainfall");
                let mut rainfall = String::new();
                io::stdin().read_line(&mut rainfall).expect("rainfall input failed");
                let rainfall:i64 = rainfall.trim().parse().expect("rainfall parse failed");

                let rainfall = rainfall * (XMAX as i64 + 1) * (YMAX as i64 + 1);

                /*println!("inport spring capacity");
                let mut spring_capacity = String::new();
                io::stdin().read_line(&mut spring_capacity).expect("spring capacity input failed");
                let spring_capacity:i64 = spring_capacity.trim().parse().expect("spring capacity parse failed");*/
                
                println!("inport ero");
                let mut ero = String::new();
                io::stdin().read_line(&mut ero).expect("ero input failed");
                let ero:f64 = ero.trim().parse().expect("ero parse failed");
        
                println!("inport speed preservation");
                let mut speed_preservation = String::new();
                io::stdin().read_line(&mut speed_preservation).expect("speed preservation input failed");
                let speed_preservation:f64 = speed_preservation.trim().parse().expect("speed preservation parse failed");
        
                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                println!("inport range (0 for no smoothing)");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:i64 = range.trim().parse().expect("range parse failed");
                

                let mut map3 = map2.clone();
                
                
                
                /*let randomxd = {
                    if spring_capacity > 1000 {
                        1
                    }
                    else {
                        1000 / spring_capacity
                    }
                };*/

                let randomxd = rainfall * 2;
//                let randomxd = rainfall/5;

                let mut lost_over_map:f64 = 0.0;


                let now = time::Instant::now();

                for i in 0..rainfall {

                    let xx:i64 = rand::thread_rng().gen_range(0..((XMAX + 1) as i64));
                    let yy:i64 = rand::thread_rng().gen_range(0..((YMAX + 1) as i64));

//                    spring3(&mut map3, xx , yy , f64::EPSILON , 8 as i64, &mut hmap, speed_preservation, ero, 0, &mut to, i+1, 0.0, &mut ts, &mut sm, range, selfbias, spring_capacity);
                    drop3(&mut map3, xx , yy , f64::EPSILON , 8 as i64, &mut hmap, speed_preservation, ero, 0, &mut to, i+1, 0.0, &mut ts, &mut sm, range, selfbias, &mut lost_over_map);
                    //println!("*dies of cringe*\n");
                    
                    if (i % randomxd == 0) && i!=0 {
                        savemap(&hmap, &hmapname);
                        savemap(&sm, &speedmapname);
                    }
                    
                    if i == 0 {
                        println!("{} / {} ({:.2}%)", i+1, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    else{
                        println!("{}{}{}{} / {} ({:.2}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    
                }
                println!("{}{}{}", clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));


                let elapsed_time = now.elapsed().as_secs_f64();
                println!("took {:.4} for an average of {:.4} ms", elapsed_time, elapsed_time/(rainfall as f64)*1000.0);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        sm[i as usize][j as usize] /= hmap[i as usize][j as usize];
                        if sm[i as usize][j as usize] == f64::NAN {
                            sm[i as usize][j as usize] = 0.0;  //dis shouwd fix godot bweaking itsewf ovew nyan
                        }
                    }
                }

                let submap3 = sub_heatmap(&map3, &hmap);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        hmap[i as usize][j as usize] = hmap[i as usize][j as usize].sqrt();
                    }
                }
                
                pushdown(&mut map3);

                
                savemap(&submap3, &subheatmapname);
                savemap(&map3, &map3name);
                savemap(&hmap, &hmapname);
                savemap(&ts, &leftmapname);
                savemap(&sm, &speedmapname);
                println!("{:.0}", avmap(&mut map3));
                println!("{:.0} lost over map ({:.2})", lost_over_map, lost_over_map / (rainfall as f64));
            }
            "d2" | "D2" => {  //drop3 iter
                let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut to = vec![ vec![0 as i64; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut ts = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut sm = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];

                println!("inport rainfall");
                let mut rainfall = String::new();
                io::stdin().read_line(&mut rainfall).expect("rainfall input failed");
                let rainfall:i64 = rainfall.trim().parse().expect("rainfall parse failed");

                let rainfall = rainfall * (XMAX as i64 + 1) * (YMAX as i64 + 1);

                /*println!("inport spring capacity");
                let mut spring_capacity = String::new();
                io::stdin().read_line(&mut spring_capacity).expect("spring capacity input failed");
                let spring_capacity:i64 = spring_capacity.trim().parse().expect("spring capacity parse failed");*/
                
                println!("inport ero");
                let mut ero = String::new();
                io::stdin().read_line(&mut ero).expect("ero input failed");
                let ero:f64 = ero.trim().parse().expect("ero parse failed");
        
                println!("inport speed preservation");
                let mut speed_preservation = String::new();
                io::stdin().read_line(&mut speed_preservation).expect("speed preservation input failed");
                let speed_preservation:f64 = speed_preservation.trim().parse().expect("speed preservation parse failed");
        

                println!("inport front bias");
                let mut frontbias = String::new();
                io::stdin().read_line(&mut frontbias).expect("front bias input failed");
                let frontbias:f64 = frontbias.trim().parse().expect("front bias parse failed");

                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                println!("inport range (0 for no smoothing)");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:i64 = range.trim().parse().expect("range parse failed");

                

                let mut map3 = map2.clone();
                
                let randomxd = rainfall * 2;
//                let randomxd = rainfall/5;

                let mut lost_over_map:f64 = 0.0;


                let now = time::Instant::now();

                for i in 0..rainfall {

                    let xx:i64 = rand::thread_rng().gen_range(0..((XMAX + 1) as i64));
                    let yy:i64 = rand::thread_rng().gen_range(0..((YMAX + 1) as i64));

//                    spring3(&mut map3, xx , yy , f64::EPSILON , 8 as i64, &mut hmap, speed_preservation, ero, 0, &mut to, i+1, 0.0, &mut ts, &mut sm, range, selfbias, spring_capacity);
                    drop3_iter1(frontbias, &mut map3, xx , yy , f64::EPSILON , 8 as i64, &mut hmap, speed_preservation, ero, 0, &mut to, i+1, &mut ts, &mut sm, range, selfbias, &mut lost_over_map);
                    //println!("*dies of cringe*\n");
                    
                    if (i % randomxd == 0) && i!=0 {
                        savemap(&hmap, &hmapname);
                        savemap(&sm, &speedmapname);
                    }
                    
                    if i == 0 {
                        println!("{} / {} ({:.2}%)", i+1, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    else{
                        println!("{}{}{}{} / {} ({:.2}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
                    }
                    
                }
                println!("{}{}{}", clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));


                let elapsed_time = now.elapsed().as_secs_f64();
                println!("took {:.4} for an average of {:.4} ms", elapsed_time, elapsed_time/(rainfall as f64)*1000.0);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        sm[i as usize][j as usize] /= hmap[i as usize][j as usize];
                        if sm[i as usize][j as usize] == f64::NAN {
                            sm[i as usize][j as usize] = 0.0;  //dis shouwd fix godot bweaking itsewf ovew nyan
                        }
                    }
                }

                let submap3 = sub_heatmap(&map3, &hmap);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        hmap[i as usize][j as usize] = hmap[i as usize][j as usize].sqrt();
                    }
                }
                
                pushdown(&mut map3);

                
                savemap(&submap3, &subheatmapname);
                savemap(&map3, &map3name);
                savemap(&hmap, &hmapname);
                savemap(&ts, &leftmapname);
                savemap(&sm, &speedmapname);
                println!("{:.0}", avmap(&mut map3));
                println!("{:.0} lost over map ({:.2})", lost_over_map, lost_over_map / (rainfall as f64));
            }
            "d3" | "D3" => {  //drop3 iter perp
                let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut ts = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut sm = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];

                println!("inport rainfall");
                let mut rainfall = String::new();
                io::stdin().read_line(&mut rainfall).expect("rainfall input failed");
                let rainfall:i64 = rainfall.trim().parse().expect("rainfall parse failed");

                let rainfall = rainfall * (XMAX as i64 + 1) * (YMAX as i64 + 1);               //rainfall aka average drops per spot
                
                println!("inport ero");
                let mut ero = String::new();
                io::stdin().read_line(&mut ero).expect("ero input failed");
                let ero:f64 = ero.trim().parse().expect("ero parse failed");
        
                println!("inport speed preservation");
                let mut speed_preservation = String::new();
                io::stdin().read_line(&mut speed_preservation).expect("speed preservation input failed");
                let speed_preservation:f64 = speed_preservation.trim().parse().expect("speed preservation parse failed");
        
                println!("inport front bias (frontside is 3.0)");
                let mut frontbias = String::new();
                io::stdin().read_line(&mut frontbias).expect("front bias input failed");
                let frontbias:f64 = frontbias.trim().parse().expect("front bias parse failed");

                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                println!("inport range (0 for no smoothing)");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:i64 = range.trim().parse().expect("range parse failed");

                

                let mut map3 = map2.clone();

                let mut lost_over_map:f64 = 0.0;


                let now = time::Instant::now();


                drop3_iter1_perp(rainfall, frontbias, &mut map3, &mut hmap, speed_preservation, ero, &mut ts, &mut sm, range, selfbias, &mut lost_over_map);


                let elapsed_time = now.elapsed().as_secs_f64();
                println!("took {:.4} for an average of {:.4} ms", elapsed_time, elapsed_time/(rainfall as f64)*1000.0);

                /*for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        sm[i as usize][j as usize] /= hmap[i as usize][j as usize];
                        if sm[i as usize][j as usize] == f64::NAN {
                            sm[i as usize][j as usize] = 0.0;  //dis shouwd fix godot bweaking itsewf ovew nyan
                        }
                    }
                }*/

                let submap3 = sub_heatmap(&map3, &hmap);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        hmap[i as usize][j as usize] = hmap[i as usize][j as usize].sqrt();
                    }
                }
                
                pushdown(&mut map3);

                
                savemap(&submap3, &subheatmapname);
                savemap(&map3, &map3name);
                savemap(&hmap, &hmapname);
                savemap(&ts, &leftmapname);
                savemap(&sm, &speedmapname);
                println!("{:.0}", avmap(&mut map3));
                println!("{:.0} lost over map ({:.2})", lost_over_map, lost_over_map / (rainfall as f64));
            },
            "d4" | "D4" => {  //drop4 (iter perp)
                let mut hmap = vec![ vec![0.0; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut ts = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];
                let mut sm = vec![ vec![0.0 as f64; (YMAX+1) as usize] ; (XMAX+1) as usize];

                println!("inport rainfall");
                let mut rainfall = String::new();
                io::stdin().read_line(&mut rainfall).expect("rainfall input failed");
                let rainfall:i64 = rainfall.trim().parse().expect("rainfall parse failed");

                let rainfall = rainfall * (XMAX as i64 + 1) * (YMAX as i64 + 1);               //rainfall aka average drops per spot
                
                println!("inport ero");
                let mut ero = String::new();
                io::stdin().read_line(&mut ero).expect("ero input failed");
                let ero:f64 = ero.trim().parse().expect("ero parse failed");
        
                println!("inport speed preservation");
                let mut speed_preservation = String::new();
                io::stdin().read_line(&mut speed_preservation).expect("speed preservation input failed");
                let speed_preservation:f64 = speed_preservation.trim().parse().expect("speed preservation parse failed");
        
                println!("inport front bias (frontside is 3.0)");
                let mut frontbias = String::new();
                io::stdin().read_line(&mut frontbias).expect("front bias input failed");
                let frontbias:f64 = frontbias.trim().parse().expect("front bias parse failed");

                println!("inport self bias");
                let mut selfbias = String::new();
                io::stdin().read_line(&mut selfbias).expect("self bias input failed");
                let selfbias:f64 = selfbias.trim().parse().expect("self bias parse failed");

                println!("inport range (0 for no smoothing)");
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("range input failed");
                let range:i64 = range.trim().parse().expect("range parse failed");

                

                let mut map3 = map2.clone();

                let mut lost_over_map:f64 = 0.0;


                let now = time::Instant::now();


                drop4_iter_perp(rainfall, frontbias, &mut map3, &mut hmap, speed_preservation, ero, &mut ts, &mut sm, range, selfbias, &mut lost_over_map);


                let elapsed_time = now.elapsed().as_secs_f64();
                println!("took {:.4} for an average of {:.4} ms", elapsed_time, elapsed_time/(rainfall as f64)*1000.0);

                /*for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        sm[i as usize][j as usize] /= hmap[i as usize][j as usize];
                        if sm[i as usize][j as usize] == f64::NAN {
                            sm[i as usize][j as usize] = 0.0;  //dis shouwd fix godot bweaking itsewf ovew nyan
                        }
                    }
                }*/

                let submap3 = sub_heatmap(&map3, &hmap);

                for i in 0..(XMAX+1) {
                    for j in 0..(YMAX+1) {
                        hmap[i as usize][j as usize] = hmap[i as usize][j as usize].sqrt();
                    }
                }
                
                pushdown(&mut map3);

                
                savemap(&submap3, &subheatmapname);
                savemap(&map3, &map3name);
                savemap(&hmap, &hmapname);
                savemap(&ts, &leftmapname);
                savemap(&sm, &speedmapname);
                println!("{:.0}", avmap(&mut map3));
                println!("{:.0} lost over map ({:.2})", lost_over_map, lost_over_map / (rainfall as f64));
            },
            _ => {
                println!("not recognized {}", mode);
            }
        }
    }

//    printmap(&map);
    /*
    ini_map(&mut map);


    println!("-------------------------------------------------");
//    printmap(&map);

    let flnm = "xdddd.txt";
    savemap(&map, &flnm);


    let flnm = "xddd2.txt";
    let hmpflnm = "hmap.txt";

    let ogmap = map.clone();
    let oghmap = hmap.clone();



    loop{

        println!("{}{}{}\n----------------------------------\n\ninport smootthrough",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));
        let mut smootthrough = String::new();
        io::stdin().read_line(&mut smootthrough).expect("xdddddddddd");
        let smootthrough:i64 = smootthrough.trim().parse().expect("xddddddddddddddd");

        println!("inport smoothing");
        let mut smoothing = String::new();
        io::stdin().read_line(&mut smoothing).expect("xdddddddddd");
        let smoothing:i64 = smoothing.trim().parse().expect("xddddddddddddddd");

        println!("inport radicalize");
        let mut rad = String::new();
        io::stdin().read_line(&mut rad).expect("xdddddddddd");
        let rad:f64 = rad.trim().parse().expect("xddddddddddddddd");

        println!("inport range");
        let mut range = String::new();
        io::stdin().read_line(&mut range).expect("xdddddddddddd");
        let range:u64 = range.trim().parse().expect("xdddddddddddddd");

        println!("inport self bias");
        let mut selfbias = String::new();
        io::stdin().read_line(&mut selfbias).expect("xdddddddddddd");
        let selfbias:f64 = selfbias.trim().parse().expect("xdddddddddddddd");



        for i in 0..smootthrough {
            for _j in 0..smoothing {
                map = smooth4(&map, range, selfbias);
            }
            map = radicalize(&map, rad);
            
            if i == 0 {
                println!("{} / {} ({}%)", i+1, smootthrough, ( (i as f64) / (smootthrough as f64) * 100.0));
            }
            else{
                println!("{}{}{}{} / {} ({}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, smootthrough, ( (i as f64) / (smootthrough as f64) * 100.0));
            }
            
        }

        savemap(&map, &flnm);

        println!("{}{}{}inport rainfall",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));
        let mut rainfall = String::new();
        io::stdin().read_line(&mut rainfall).expect("xdddddddddd");
        let rainfall:i64 = rainfall.trim().parse().expect("xddddddddddddddd");

        println!("inport ero");
        let mut ero = String::new();
        io::stdin().read_line(&mut ero).expect("xdddddddddd");
        let ero:f64 = ero.trim().parse().expect("xddddddddddddddd");

        println!("inport material loss");
        let mut loss = String::new();
        io::stdin().read_line(&mut loss).expect("xdddddddddd");
        let loss:f64 = loss.trim().parse().expect("xddddddddddddddd");

        println!("inport self bias");
        let mut selfbias = String::new();
        io::stdin().read_line(&mut selfbias).expect("xdddddddddddd");
        let selfbias:f64 = selfbias.trim().parse().expect("xdddddddddddddd");
        

        for i in 0..rainfall {
            if (i != 0) && ( i % (((XMAX + 1) * (YMAX + 1)) as i64) == 0){
                map = smooth4(&map, range, selfbias);
            }
            drop1(&mut map, ero, loss, &mut hmap);
            
            
            if i == 0 {
                println!("{} / {} ({}%)", i+1, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
            }
            else{
                println!("{}{}{}{} / {} ({}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, rainfall, ( (i as f64) / (rainfall as f64) * 100.0));
            }
        }
        
        map = smooth4(&mut map, range, selfbias);

        savemap(&map, &flnm);
        savemap(&hmap, &hmpflnm);



        map = ogmap.clone();
        hmap = oghmap.clone();
    }
    */
}

/*
fn printmap (map: &Vec<Vec<f64>>){
    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            print!("{} ",map[i as usize][j as usize]);
        }
        println!(" ");
    }
}
*/

fn interpolate (a0:f64, a1:f64, w:f64) -> f64 {
    //(a1 - a0) * w + a0
    //return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

fn random_gradient (ix:i64, iy:i64) -> [f64; 2] {
//    println!("rG with {} {}", ix, iy);
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
//    println!("rG return {:?}", v);
    v
}

fn dot_grid_gradient(ix:i64, iy:i64, x:f64, y:f64) -> f64 {
//    println!("dGG with {} {} {} {}", ix, iy, x, y);
    let gradient = random_gradient(ix, iy);
    let dx = x - (ix as f64);
    let dy = y - (iy as f64);
//    println!("{} {}", dx, dy);
//    println!("{} {}", gradient[0], gradient[1]);
//    println!("dGG return {}", (dx*gradient[0] + dy*gradient[1]));
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
    
//    interpolate(ix0, ix1, sy) * 0.5 + 0.5
    interpolate(ix0, ix1, sy)
}

fn perlin_scaling_abs_sqrt (x:f64) -> f64 {
//    0.5 * (3.1 * x + 4.73).sin() + 0.5
//    x.abs()
    x.abs().sqrt()
//    (x+0.5) / ((x+0.5).abs() + 1.0) + 0.5
}

fn perlin_scaling_abs (x:f64) -> f64{
    x.abs()
}

fn perlin_scaling_x (x:f64) -> f64 {
    x
}


fn avmap (map: &Vec<Vec<f64>> ) -> f64 {
    let mut x = 0.0;
    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            x += map[i as usize][j as usize];
        }
    }
    x / (((XMAX+1) * (YMAX+1))as f64)
}

fn sub_heatmap(map: &Vec<Vec<f64>> , hmap: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut ret = map.clone();
    for i in 0..(XMAX+1)
    {
        for j in 0..(YMAX+1)
        {
            ret[i as usize][j as usize]-=hmap[i as usize][j as usize];
            ret[i as usize][j as usize] = ret[i as usize][j as usize].max(0.0);
        }
    }
    ret
}

fn savemap (map: &Vec<Vec<f64>>, filename: &str) {
    let mut output = File::create(filename)
        .expect("xd robienie pliku nie dziala");
    
    
    output.write((XMAX+1).to_string().as_bytes()).expect("dxd");
    output.write(b";").expect("dxd");
    output.write((YMAX+1).to_string().as_bytes()).expect("dxd");
    output.write(b"\n").expect("dxd");

    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            output.write(map[i as usize][j as usize].to_string().as_bytes()).expect("dxd");
            output.write(b";").expect("dxd");
        }
        output.write(b"\n")
            .expect("dxd");
    }
}

fn ini_map (map: &mut Vec<Vec<f64>>){
    for i in 0..(XMAX+1) {
        for j in 0..(YMAX+1) {
            map[i as usize][j as usize] = rand::thread_rng().gen_range(ZMININI..ZMAX);
        }
    }
}



fn get_map_val(x:i64 , y:i64,map: &Vec<Vec<f64>>) -> f64{
//    let mut ret=0;
    let mid:Vec<f64>;
    match map.get(x as usize) {
        Some(out) => { mid = out.to_vec() }
        None => {return 0.0;} 
    }
    match mid.get(y as usize ) {
        Some(out) => {return *out;}
        None => {return 0.0;}
        
    }
}

fn check_coords(x:i64 , y:i64) -> bool{
    return x >= 0 && (x as u64) <= XMAX && y >= 0 && (y as u64) <= YMAX
}


//---------------------------------------------------------------------------------------------------------------------------------------------------
fn max_capacity (speed:f64) -> f64 {//                                                      dis exists just two make it easiew two change teh fowmuwa
//    speed
    speed.sqrt() + 1.0
}
//---------------------------------------------------------------------------------------------------------------------------------------------------


/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

