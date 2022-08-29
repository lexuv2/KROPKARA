use crate::XMAX;
use crate::YMAX;


use crate::check_coords;
use crate::smootharea2;
use crate::max_capacity;
use crate::smootharea;
use crate::get_map_val;


use rand::Rng;
use ansi_control::*;



pub fn drop3(map: &mut Vec<Vec<f64>>, x:i64, y:i64, speed:f64, dir:i64, dropmap: &mut Vec<Vec<f64>>, speed_preservation:f64, erosion:f64, life:i64, odw: &mut Vec<Vec<i64>>,moment:i64, stored:f64, ts:&mut Vec<Vec<f64>>, speedmap:&mut Vec<Vec<f64>>, range:i64, selfbias:f64, lost_over_map: &mut f64) {
    let mut st2 = stored;


    
    if check_coords(x, y) == false {
        *lost_over_map += st2;
        return;
    }


    let dropdebug:bool = false;

    if dropdebug {
        println!("have {},{} out of {}",st2, stored, max_capacity(speed));
    }


    dropmap[x as usize][y as usize] += 1.0;
    speedmap[x as usize][y as usize] += speed;

    if odw[x as usize][y as usize] == moment {                                                          //loop
        map[x as usize][y as usize] += st2;
        ts[x as usize][y as usize] += st2;
        if dropdebug {
            println!("drop loop {}", st2);
        }

        if range > 0 {
            smootharea(map, range, selfbias, x, y);
        }
        return;
    }
    else {
        odw[x as usize][y as usize] = moment;
    }
//                                                                                                                                  0 1 2
//                                                                                                                                  7 8 3
//                                                                                                                                  6 5 4
    let surr_x = [x-1, x, x+1, x+1, x+1, x, x-1, x-1];
    let surr_y = [y-1, y-1, y-1, y, y+1, y+1, y+1, y];

    let front = 3.2;
    let frontside = 3.1 + f64::EPSILON;
    let side = 1.8;
    let backside = 1.0;
    let back = f64::EPSILON;
    let ffsbb = [front, frontside, side, backside, back, backside, side, frontside];
    //           0      1          2     3         4     5         6     7
    let mut surr_multiplier = [1.0; 8 as usize];
    if dir != 8 {
        for i in (0 as i64)..(8 as i64) {
            surr_multiplier[i as usize] = ffsbb[((i-dir+64)%8) as usize];
        }
    }

    let mut surr_height = [0.0; 9 as usize];
    let mut surr_lowest:f64 = f64::MAX;
    let mut surr_lowest_dir:i64 = 8;
    let mut highest_prio = f64::MIN;
    let mut highest_prio_dir;  highest_prio_dir = -1;                                             let _ = highest_prio_dir;

    for i in (0 as i64)..(8 as i64) {
        surr_height[i as usize] = get_map_val(surr_x[i as usize], surr_y[i as usize], map);
        if surr_height[i as usize] < surr_lowest {
            surr_lowest = surr_height[i as usize];
            surr_lowest_dir = i;
        }
    }
    let local_height = get_map_val(x, y, map);

    if surr_lowest > local_height {                                                   
        if surr_lowest - local_height >= st2 {                                        
            map[x as usize][y as usize] += st2;
            ts[x as usize][y as usize] += st2;

            if dropdebug {
                println!("drop hole {}", st2);
            }
            
            if range > 0 {
                smootharea(map, range, selfbias, x, y);
            }
            return;
        }
        else {                                                                       
            map[x as usize][y as usize] += surr_lowest - local_height;
            ts[x as usize][y as usize] += surr_lowest - local_height;
            if dropdebug {
                println!("drop outbury {}", surr_lowest - local_height);
            }
            highest_prio_dir = surr_lowest_dir;
            st2 -= surr_lowest - local_height;
        }
    }
    else{                                                                               
        for i in (0 as i64)..(8 as i64) {                                               
            if highest_prio < (local_height - surr_height[i as usize]) * surr_multiplier[i as usize] {
                highest_prio_dir = i;
                highest_prio = (local_height - surr_height[i as usize]) * surr_multiplier[i as usize];
            }
        }                                                                              

    }



    let mut sp2 = speed * speed_preservation * (front / surr_multiplier[highest_prio_dir as usize]) + local_height - surr_height[highest_prio_dir as usize] + f64::EPSILON;
    //println!("{sp2}");
    if surr_multiplier[highest_prio_dir as usize] == f64::EPSILON {
        sp2 = f64::EPSILON;
    }
    //println!("{sp2}");
    if surr_height[highest_prio_dir as usize] < 0.0001 {
        sp2 = speed * speed_preservation * (front / surr_multiplier[highest_prio_dir as usize]) + f64::EPSILON;
    }
    //println!("{sp2}");
    if (dir - highest_prio_dir).abs() == 4 {
        sp2 = f64::EPSILON;
        //println!("allegedly fixed backout");
    }
    //println!("{sp2}");
                                //speed at which teh dwop wiww weave teh tiwe~ teh epsiwon is dewe to pwevent speed being zewo in case we faww into a howe and speed pwesewvation is set to 0 uwu 

    /*if sp2 > 10000.0 {
        println!("OOPSIE WOOPSIE!! Uwu We made a fucky wucky!! A wittle fucko boingo! The code monkeys at our headquarters are working VEWY HAWD to fix this!");
        println!("{} , {} , {} , {} ", dir, highest_prio_dir, x, y);
        println!("x");
    }*/

    let target_stored = max_capacity(sp2);


    if target_stored <= st2 {                                                           //some has two be dwopped
        map[x as usize][y as usize] += st2 - target_stored;
        ts[x as usize][y as usize] += st2 - target_stored;

        if dropdebug {
            println!("drop slowdown {}", st2 - target_stored);
        }

        st2 = target_stored;
    }
    else{                                                                               //some can be picked up
        
        let change = sp2 * erosion;
        let change = change.min(target_stored - st2);
        let change = change.min(local_height - surr_height[highest_prio_dir as usize]);
        let change = change.max(0.0);

        map[x as usize][y as usize] -= change;
        ts[x as usize][y as usize] -= change;

        st2 += change;

        if dropdebug {
            println!("pick up {}, st2 is now {}", change, st2);
        }
    }


    //println!(" go {} with {} at a high speed ({})", highest_prio_dir, st2, sp2);

    drop3(map,surr_x[highest_prio_dir as usize], surr_y[highest_prio_dir as usize],sp2, highest_prio_dir, dropmap,speed_preservation,erosion,life+1,odw,moment,st2,ts,speedmap, range, selfbias, lost_over_map);


    if range > 0 {
        smootharea2(map, range, selfbias, x, y);
    }

    /*
    if range > 0 {
        smootharea(map, range, selfbias, x, y);
    }
    */
}

pub fn drop3_iter1 (front:f64, map: &mut Vec<Vec<f64>>, drop_x:i64, drop_y:i64, mut speed:f64, drop_dir:i64, dropmap: &mut Vec<Vec<f64>>, speed_preservation:f64, erosion:f64, life:i64, odw: &mut Vec<Vec<i64>>,moment:i64, ts:&mut Vec<Vec<f64>>, speedmap:&mut Vec<Vec<f64>>, range:i64, selfbias:f64, lost_over_map: &mut f64) {
    
    let mut st:f64 = 0.0;
    let mut x = drop_x;
    let mut y = drop_y;
    let mut dir = drop_dir;


    let dropdebug:bool = false;
    let godebug:bool = false;

    let mut surr_x = [0 as i64; 8];     let _ = surr_x[7];
    let mut surr_y = [0 as i64; 8];     let _ = surr_y[7];

    //let front = 4.0;
    let frontside = 3.0;
    let side = 2.0;
    let backside = 1.0;
    let back = f64::EPSILON;
    
    let ffsbb = [front, frontside, side, backside, back, backside, side, frontside];
    //           0      1          2     3         4     5         6     7
    /*let ffsbb = [1.0; 9 as usize];
    let front = 1.0;
    let frontside = 1.0;
    let side = 1.0;
    let backside = 1.0;
    let back = 1.0;*/

    let mut surr_multiplier = [1.0 as f64; 8];

    let mut surr_height = [0.0; 9 as usize];
    let mut surr_lowest:f64;
    let mut surr_lowest_dir:i64 = 8;
    let mut highest_prio:f64;
    let mut highest_prio_dir;  highest_prio_dir = -1;                                             let _ = highest_prio_dir;
    let mut local_height:f64;
    let mut sp2:f64;
    let mut target_stored:f64;
    let mut change:f64;

    
    let mut smooth_stack = vec![(0,0); 0];
    
    if godebug | dropdebug {
        println!("-----------------------------------------------------------------");
    }

    'drop_life: loop {
        if godebug {
            println!("in {} {}", x, y);
        }

        if check_coords(x, y) == false {
            *lost_over_map += st;
            break 'drop_life;
        }

        if dropdebug {
            println!("have {} out of {}",st, max_capacity(speed));
        }

        dropmap[x as usize][y as usize] += 1.0;
        speedmap[x as usize][y as usize] += speed;


        if odw[x as usize][y as usize] == moment {                                                          //loop
            map[x as usize][y as usize] += st;
            ts[x as usize][y as usize] += st;
            if dropdebug {
                println!("drop loop {}", st);
            }
    
            if range > 0 {
                smootharea(map, range, selfbias, x, y);
            }
            break 'drop_life;
        }
        else {
            odw[x as usize][y as usize] = moment;
        }
//                                                                                                                                  0 1 2
//                                                                                                                                  7 8 3
//                                                                                                                                  6 5 4
        surr_x = [x-1, x, x+1, x+1, x+1, x, x-1, x-1];
        surr_y = [y-1, y-1, y-1, y, y+1, y+1, y+1, y];

        if dir != 8 {
            for i in (0 as i64)..(8 as i64) {
                surr_multiplier[i as usize] = ffsbb[((i-dir+64)%8) as usize];
            }
        }

        surr_lowest = f64::MAX;
        for i in (0 as i64)..(8 as i64) {
            surr_height[i as usize] = get_map_val(surr_x[i as usize], surr_y[i as usize], map);
            if surr_height[i as usize] < surr_lowest {
                surr_lowest = surr_height[i as usize];
                surr_lowest_dir = i;
            }
        }
        local_height = get_map_val(x, y, map);


        highest_prio = f64::MIN;
        if surr_lowest > local_height {                                                   
            if surr_lowest - local_height >= st {                                        
                map[x as usize][y as usize] += st;
                ts[x as usize][y as usize] += st;
    
                if dropdebug {
                    println!("drop hole {}", st);
                }
                
                if range > 0 {
                    smootharea(map, range, selfbias, x, y);
                }
                break 'drop_life;
            }
            else {                                                                       
                map[x as usize][y as usize] += surr_lowest - local_height;
                ts[x as usize][y as usize] += surr_lowest - local_height;
                if dropdebug {
                    println!("drop outbury {}", surr_lowest - local_height);
                }
                highest_prio_dir = surr_lowest_dir;
                st -= surr_lowest - local_height;
            }
        }
        else{                                                                               
            for i in (0 as i64)..(8 as i64) {                                               
                if highest_prio < (local_height - surr_height[i as usize]) * surr_multiplier[i as usize] {
                    highest_prio_dir = i;
                    highest_prio = (local_height - surr_height[i as usize]) * surr_multiplier[i as usize];
                }
            }                                                                              
    
        }

        sp2 = speed * speed_preservation * (front / surr_multiplier[highest_prio_dir as usize]) + local_height - surr_height[highest_prio_dir as usize] + f64::EPSILON;

        if surr_multiplier[highest_prio_dir as usize] == f64::EPSILON {
            sp2 = f64::EPSILON;
        }
        else if (dir - highest_prio_dir).abs() == 4 {
            sp2 = f64::EPSILON;
        }
        if surr_height[highest_prio_dir as usize] < 0.0001 {
            sp2 = speed * speed_preservation * (front / surr_multiplier[highest_prio_dir as usize]) + f64::EPSILON;
        }

        target_stored = max_capacity(sp2);
        
        if target_stored <= st {                                                           //some has two be dwopped
            map[x as usize][y as usize] += st - target_stored;
            ts[x as usize][y as usize] += st - target_stored;
    
            if dropdebug {
                println!("drop slowdown {}", st - target_stored);
            }
    
            st = target_stored;
        }
        else{                                                                               //some can be picked up
            
            change = sp2 * erosion;
            change = change.min(target_stored - st);
            change = change.min(local_height - surr_height[highest_prio_dir as usize]);
            change = change.max(0.0);
    
            map[x as usize][y as usize] -= change;
            ts[x as usize][y as usize] -= change;
    
            st += change;
    
            if dropdebug {
                println!("pick up {}, st2 is now {}", change, st);
            }
        }

        //this was the recursion part before
        if range > 0 {
            smooth_stack.push((x,y));
        }
        x = surr_x[highest_prio_dir as usize];
        y = surr_y[highest_prio_dir as usize];
        dir = highest_prio_dir;
        speed = sp2;

    }

    if range > 0 {
        while let Some(top) = smooth_stack.pop() {
            smootharea2(map, range, selfbias, top.0, top.1);
        }
    }
    
}

pub fn drop3_iter1_perp (rounds:i64, front:f64, map: &mut Vec<Vec<f64>>, dropmap: &mut Vec<Vec<f64>>, speed_preservation:f64, erosion:f64, ts:&mut Vec<Vec<f64>>, speedmap:&mut Vec<Vec<f64>>, range:i64, selfbias:f64, lost_over_map: &mut f64) {
    

    let frontside = 3.0;
    let side = 2.0;
    let backside = 1.0;
    let back = f64::EPSILON;
    let ffsbb = [front, frontside, side, backside, back, backside, side, frontside];
    //           0      1          2     3         4     5         6     7
    //-------------------------------------------------------------------------------setup static prio stuff

    let dropdebug:bool = false;
    let godebug:bool = false;
    //-------------------------------------------------------------------------------setup static debug stuff

    let mut odw = vec![ vec![0 as i64; (YMAX+1) as usize] ; (XMAX+1) as usize];

    for i in 0..(rounds+1) {//-------------------------------------------------------individual drops (this is the difference between drop3_iter1 and drop3_iter1_perp)
        

        if i % 10 == 0 {//-----------------------------------------------------------progress
            if i == 0 {
                println!("{} / {} ({:.2}%)", i+1, rounds, ( (i as f64) / (rounds as f64) * 100.0));
            }
            else{
                println!("{}{}{}{} / {} ({:.2}%)",clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both), i, rounds, ( (i as f64) / (rounds as f64) * 100.0));
            }
        }

        

        let mut st:f64 = 0.0;
        let mut x:i64 = rand::thread_rng().gen_range(0..((XMAX + 1) as i64));
        let mut y:i64 = rand::thread_rng().gen_range(0..((YMAX + 1) as i64));
        let mut dir = 8;
        let mut surr_x = [0 as i64; 8];     let _ = surr_x[7];
        let mut surr_y = [0 as i64; 8];     let _ = surr_y[7];
        let mut surr_multiplier = [1.0 as f64; 8];
        let mut surr_height = [0.0; 9 as usize];
        let mut surr_lowest:f64;
        let mut surr_lowest_dir:i64 = 8;
        let mut highest_prio:f64;
        let mut highest_prio_dir;  highest_prio_dir = -1;                                             let _ = highest_prio_dir;//<-- this fixes rust warnings
        let mut local_height:f64;
        let mut sp2:f64;
        let mut target_stored:f64;
        let mut change:f64;
        //---------------------------------------------------------------------------setup stuff needed for drop fall
        
        let mut smooth_stack = vec![(0,0); 0];
        //---------------------------------------------------------------------------this is used only if range > 0
        
        if godebug | dropdebug {
            println!("-----------------------------------------------------------------");
        }

        let mut speed:f64 = f64::EPSILON;
        //---------------------------------------------------------------------------this might (should) be adjusted later, for now its a placeholder that works
        

        'drop_life: loop {//---------------------------------------------------------actuall drop code
            if godebug {
                println!("in {} {}", x, y);
            }

            if check_coords(x, y) == false {
                *lost_over_map += st;
                break 'drop_life;
            }//----------------------------------------------------------------------this handles drop falling over the flat earth

            if dropdebug {
                println!("have {} out of {}",st, max_capacity(speed));
            }

            dropmap[x as usize][y as usize] += 1.0;
            


            if odw[x as usize][y as usize] == i+1 {
                map[x as usize][y as usize] += st;
                ts[x as usize][y as usize] += st;
                if dropdebug {
                    println!("drop loop {}", st);
                }
        
                if range > 0 {
                    smootharea(map, range, selfbias, x, y);
                }
                break 'drop_life;
            }
            else {
                odw[x as usize][y as usize] = i+1;
            }//----------------------------------------------------------------------this handles drop falling into a loop
    //                                                                                                                                  0 1 2
    //                                                                                                                                  7 8 3
    //                                                                                                                                  6 5 4
            surr_x = [x-1, x, x+1, x+1, x+1, x, x-1, x-1];
            surr_y = [y-1, y-1, y-1, y, y+1, y+1, y+1, y];
            //-----------------------------------------------------------------------this is just some stuff that makes later code cleaner and maybe marginally faster (less ifs)

            if dir != 8 {
                for i in (0 as i64)..(8 as i64) {
                    surr_multiplier[i as usize] = ffsbb[((i-dir+64)%8) as usize];
                }
            }//----------------------------------------------------------------------this handles direction prio

            surr_lowest = f64::MAX;
            for i in (0 as i64)..(8 as i64) {
                surr_height[i as usize] = get_map_val(surr_x[i as usize], surr_y[i as usize], map);
                if surr_height[i as usize] < surr_lowest {
                    surr_lowest = surr_height[i as usize];
                    surr_lowest_dir = i;
                }
            }//----------------------------------------------------------------------this decides where to fall (if fall at all)


            local_height = get_map_val(x, y, map);

            highest_prio = f64::MIN;
            if surr_lowest > local_height {                                                   
                if surr_lowest - local_height >= st {                                        
                    map[x as usize][y as usize] += st;
                    ts[x as usize][y as usize] += st;
        
                    if dropdebug {
                        println!("drop hole {}", st);
                    }
                    
                    if range > 0 {
                        smootharea(map, range, selfbias, x, y);
                    }
                    break 'drop_life;
                }//------------------------------------------------------------------this handles drop being in 1x1 hole it cant escape
                else {                                                                       
                    map[x as usize][y as usize] += surr_lowest - local_height;
                    ts[x as usize][y as usize] += surr_lowest - local_height;
                    if dropdebug {
                        println!("drop outbury {}", surr_lowest - local_height);
                    }
                    highest_prio_dir = surr_lowest_dir;
                    st -= surr_lowest - local_height;
                }//------------------------------------------------------------------this handles drop being in a 1x1 hole it can escape
            }//----------------------------------------------------------------------this handles drop being in a 1x1 hole
            else{                                                                               
                for i in (0 as i64)..(8 as i64) {                                               
                    if highest_prio < (local_height - surr_height[i as usize]) * surr_multiplier[i as usize] {
                        highest_prio_dir = i;
                        highest_prio = (local_height - surr_height[i as usize]) * surr_multiplier[i as usize];
                    }
                }                                                                              
            }//----------------------------------------------------------------------this handles drop being able to fall freely

            sp2 = speed * speed_preservation * (front / surr_multiplier[highest_prio_dir as usize]) + local_height - surr_height[highest_prio_dir as usize] + f64::EPSILON;
            speedmap[x as usize][y as usize] += sp2;
            //-----------------------------------------------------------------------this handles exit speed

            if surr_multiplier[highest_prio_dir as usize] == f64::EPSILON {
                sp2 = f64::EPSILON;
            }
            else if (dir - highest_prio_dir).abs() == 4 {
                sp2 = f64::EPSILON;
            }
            if surr_height[highest_prio_dir as usize] < 0.0001 {
                sp2 = speed * speed_preservation * (front / surr_multiplier[highest_prio_dir as usize]) + f64::EPSILON;
            }
            //-----------------------------------------------------------------------this is some deal with the devil stuff that has no reason to work but drop breaks without it so i keep it here

            target_stored = max_capacity(sp2);
            //-----------------------------------------------------------------------actuall erosion setup
            
            if target_stored <= st {
                map[x as usize][y as usize] += st - target_stored;
                ts[x as usize][y as usize] += st - target_stored;
        
                if dropdebug {
                    println!("drop slowdown {}", st - target_stored);
                }
        
                st = target_stored;
            }//----------------------------------------------------------------------this handles dropping some rocks because drop is oversaturated
            else{
                
                change = sp2 * erosion;
                change = change.min(target_stored - st);
                change = change.min(local_height - surr_height[highest_prio_dir as usize]);
                change = change.max(0.0);
                //-------------------------------------------------------------------this could be 1 line in theory but rust breaks it and just returns 0.0 for some reason and i dont want to debug it
        
                map[x as usize][y as usize] -= change;
                ts[x as usize][y as usize] -= change;
        
                st += change;
        
                if dropdebug {
                    println!("pick up {}, st2 is now {}", change, st);
                }
            }//----------------------------------------------------------------------this handles picking up some rocks because drop is undersaturated

            
            if range > 0 {
                smooth_stack.push((x,y));
            }//----------------------------------------------------------------------this adds cords to smooth, was implicit before with recursion

            x = surr_x[highest_prio_dir as usize];
            y = surr_y[highest_prio_dir as usize];
            dir = highest_prio_dir;
            speed = sp2;
            //-----------------------------------------------------------------------this is transition to next location

        }

        if range > 0 {
            while let Some(top) = smooth_stack.pop() {
                smootharea2(map, range, selfbias, top.0, top.1);
            }
        }//--------------------------------------------------------------------------this handles smoothing, prob irrel
    }

    println!("{}{}{}", clear_line(Pos::Both), move_line(-1), clear_line(Pos::Both));
    //-------------------------------------------------------------------------------this deletes progress after all drops are dropped
    
}

