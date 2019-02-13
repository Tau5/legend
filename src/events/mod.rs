pub mod triggers;
use files::maps::*;
use files;
use game_loop;
use tools;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use gameplay::renderization::render;
pub fn run_event(name: String, window: &pancurses::Window, world: &World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize, config: files::config::Config, vars: &mut Vec<i16>) -> (u8, String) { //Executes a specific event
    let mut return_code = 0;
    let mut message: String = "".to_string();
    /*
                Return code list
    -----------------------------------------
    | ID | Description                      |
    -----------------------------------------
    | 1  | End current game_loop            |
    -----------------------------------------
    */
    for i in world.events.iter() {
        if i.0==name {
            for c in i.1.iter() {
                if c.0 == "warp"{
                    let map = read_worldmap(c.clone().1); 
                    return_code = 1; //Set return code to kill the current game_loop
                    game_loop(map.clone(), window, map.world, map.char_map, map.collision_map, false, 0,0, c.clone().1, config.clone(), vars); //Start the game_loop in the new map
                }
                if c.0=="warp_custom_coor"{
                    let map = read_worldmap(c.clone().1); 
                    return_code = 1; //Set return code to kill the current game_loop
                    game_loop(map.clone(), window, map.world, map.char_map, map.collision_map, true, c.2 as usize,c.3 as usize, c.clone().1, config.clone(), vars); //Start the game_loop in the new map
                }
                if c.0 == "setw"{
                    let index = tools::get_loc(world.clone().world, c.2, c.3);
                    //std::fs::write("./log", format!("{:?}", c));
                    world_map[index] = c.4;
                }
                if c.0 == "setc"{
                    let index = tools::get_loc_coll(world.clone().collision_map, c.2, c.3);
                    collision_map[index] = c.4 as u8;
                }
                if c.0 == "msg" {
                    return_code = 0;
                    render(&window, &world.clone().world, tools::get_line_count(&world.clone().world), x, y, &world.clone().char_map, '*', c.clone().1, world.clone());
                    message = c.clone().1; 
                }
                if c.0 == "set" {
                    if c.2 >= vars.len() as u32 {
                        for _i in vars.len()..(c.2 as usize)+1 {
                            vars.push(0);
                        }
                        
                    }
                    vars[c.2 as usize] = c.3 as i16;
                }
                if c.0 == "if" {
                    if vars.len() <= c.2 as usize {
                        if c.3 as i16 == 0 {
                            continue;
                        } else {
                            break;
                        }
                    } else if vars[c.2 as usize] == c.3 as i16 {
                        continue;
                    } else {
                        break;
                    }
                }
                if c.0 == "movie" {
                 let mut content = String::new();
                 let mut file = File::open(Path::new(&files::path::get_path(format!("{}", c.1).to_string()))).unwrap();
                 file.read_to_string(&mut content).expect("Could not find movie file");
                 window.clear();
                 window.mv(0,0);
                 window.printw(content);
                 window.printw("\nPress any key to continue");
                 window.getch();
                }
                if c.0 == "wait" {
                    window.getch();
                }
            }
        }
    }
    (return_code, message)
}