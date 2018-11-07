extern crate pancurses;
extern crate serde_json;
use std::io::prelude::*;
use std::fs::File;
extern crate serde;

#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    author: String,
    initial_map: String
}

#[derive(Serialize, Deserialize, Clone)]
struct World {
    world: Vec<u32>,
    collision_map: Vec<u8>,
    char_map: Vec<char>,
    spawn: [usize; 2],
    events: Vec<(String, Vec<(String, String, u32, u32, u32, u32, u32)>)>,
    triggers: Vec<(usize, usize, String, usize, usize, usize, usize, usize)>
}
use std::path::Path;
use pancurses::{initscr, endwin, Input, noecho};
fn get_path(path_from_cwd: String)-> String{ //Get the path where the executable is located
    let filen =  path_from_cwd;
    let path = std::env::current_exe().unwrap();
    
    format!("{}{}", path.parent().unwrap().display(), filen)
}
fn read_worldmap(filename: String) -> World { //Get a World structure from a map file
    let mut file = File::open(Path::new(&get_path(format!("{}{}", "/game/", filename).to_string()))).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not find game world file");
    let world_file: World = serde_json::from_str(&content).unwrap();
    world_file
}

fn start() {
    //Read game manifest and load initial_map
    let mut file = File::open(Path::new(&get_path("/game/game.json".to_string()))).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not find game config file");
    let config: Config = serde_json::from_str(&content).unwrap();
    let mut file = File::open(Path::new(&get_path(format!("{}{}", "/game/", config.initial_map).to_string()))).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not find game world file");
    let mut world_file: World = serde_json::from_str(&content).unwrap();
    let mut world = world_file.world.clone();
    let mut char_map = world_file.char_map.clone();
    let mut collision_map = world_file.collision_map.clone();
        //Start curses mode
        let window = initscr();
        window.refresh();
        window.keypad(true);
        noecho();
        pancurses::curs_set(0);
    window.printw(format!("{} by {}\n\n", config.name, config.author));
    window.printw("INSTRUCTIONS: Use Q to exit, Use the arrow keys or WASD to move, Use Z or K to interact.\nPress a key to continue");
    window.getch();
    //Start game loop with the initial_map
    game_loop(world_file.clone(), &window, world, char_map, collision_map);
}
fn game_loop(mut world_file: World, window: &pancurses::Window, mut world:Vec<u32>, mut char_map: Vec<char>, mut collision_map:Vec<u8>) {
    let mut x = world_file.spawn[0];
    let mut y = world_file.spawn[1];
    let mut facing: u8 = 1;
    render(&window, &world, get_line_count(&world), x, y, &char_map, '*'); //Render the map
    loop {
        match window.getch() {
            Some(Input::KeyLeft)|Some(Input::Character('a')) => { 
                facing = 3;
                if x>0&&check_collision(x-1, y, &collision_map) {
                    x-=1;
                }
            },
            Some(Input::KeyRight)|Some(Input::Character('d')) => { 
                facing = 1;
                if check_collision(x+1, y, &collision_map) {
                    x+=1;
                } 
            },
            Some(Input::KeyUp)|Some(Input::Character('w')) => { 
                facing = 0;
                if y>0&&check_collision(x, y-1, &collision_map) {
                    y-=1;
                }
            },
            Some(Input::KeyDown)|Some(Input::Character('s')) => { 
                facing = 2;
                if check_collision(x, y+1, &collision_map) {
                    y+=1;
                }
                    
            },
            Some(Input::KeyExit)|Some(Input::Character('q')) => {
                break
            },
            Some(Input::Character('k'))|Some(Input::Character('z')) => {
                let code = check_interactable_triggers(&window,&world_file,&mut world, &mut collision_map, x, y, facing); //Read for interact triggers
                if code==1 { break ; }
            }
            Some(_input) => {continue},
            None => {continue}
        }
        window.clear();
        let code = check_triggers(&window,&world_file,&mut world, &mut collision_map, x, y); //Read for triggers
        if code==1 { break ; }
        render(&window, &world, get_line_count(&world), x, y, &char_map, '*');
        
    }
    
}
pub fn main() {
    start();
    endwin();
}
fn run_event(name: String, window: &pancurses::Window, world:&World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize) -> u8 { //Executes a specific event
    let mut return_code = 0;
    /*
        Code list:
            1: End current game_loop (Succefull)
    */
    for i in world.events.iter() {
        if(i.0==name){
            for c in i.1.iter() {
                if(c.0=="warp"){
                    let map = read_worldmap(c.clone().1); 
                    return_code = 1; //Set return code to kill the current game_loop
                    game_loop(map.clone(), window, map.world, map.char_map, map.collision_map); //Start the game_loop in the new map
                }
                if(c.0=="setw"){
                    let index = get_loc(world.clone().world, c.2, c.3);
                    //std::fs::write("./log", format!("{:?}", c));
                    world_map[index] = c.4;
                }
                if(c.0=="setc"){
                    let index = get_loc(world.clone().world, c.2, c.3);
                    collision_map[index] = c.4 as u8;
                }
            }
        }
    }
    return_code    
}
fn check_interactable_triggers(window: &pancurses::Window, world:&World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize, facing: u8) -> u8 { 
    let mut return_code = 0;
    let mut face_x = x;
    let mut face_y = y;
    match facing {
        0 => {
            face_y-=1
        },
        1 => {
            face_x+=1
        },
        2 => {
            face_y+=1
        },
        3 => {
            face_x-=1
        },
        _ => {}
    }
    for i in world.triggers.iter() { //Iterate trough triggers to check if a events must be ran
        if i.0==face_x&&i.1==face_y&&i.3==1 {
            return_code = run_event(i.clone().2, window, &world, world_map, collision_map, x, y);
        }
    }
    return_code
}
fn check_triggers(window: &pancurses::Window, world:&World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize) -> u8 { 
    let mut return_code = 0;
    for i in world.triggers.iter() { //Iterate trough triggers to check if a events must be ran
        if i.0==x&&i.1==y&&i.3==0 {
            return_code = run_event(i.clone().2, window, &world, world_map, collision_map, x, y);
        }
    }
    return_code
}
fn check_collision(x:usize, y:usize, collision_map: &Vec<u8>)-> bool{ //Check for collisions
    let line = get_collision_line(&collision_map, y as u32);
   if line[x]==0{
       true
   } else {
       false
   }
}
fn get_line_count(world: &Vec<u32>) -> u32 { //Get the lines of the map
    let mut count = 0;
    for i in world.iter() {
        if *i==0 {count+=1;}
    }
    count
}
fn get_collision_line(world: &Vec<u8>, line_number: u32)-> Vec<u8>{ //Get a Vector of a line in the collision_map
    let mut line_index: u32 = 0;
    let mut line: Vec<u8> = Vec::new();
    for i in world.iter() {
        if *i==2 { line_index+=1; } else if line_index==line_number {
            line.push(*i);
        }
    }
    line
}
fn get_line(world: Vec<u32>, line_number: u32)-> Vec<u32>{ //Get a Vector of a line in the world map
    let mut line_index: u32 = 0;
    let mut line: Vec<u32> = Vec::new();
    for i in world.iter() {
        if *i==0 { line_index+=1; } else if line_index==line_number {
            line.push(*i);
        }
    }
    line
}
fn get_loc(world: Vec<u32>, x: u32, y:u32)-> usize{ //Get the index
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    let mut index: usize = 0;
    for (i, n) in world.iter().enumerate() {
        if col==x&&row==y {
            index = i;
            break;
        }
        if *n==0 {
            row+=1;
            col=0;
        } else {
            col+=1;
        }
        
        
    }
    index
}
fn get_loc_coll(world: Vec<u8>, x: u32, y:u32)-> usize{ //Get the index
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    let mut index: usize = 0;
    for (i, n) in world.iter().enumerate() {
        if col==x&&row==y {
            index = i;
            break;
        }
        if *n==2 {
            row+=1;
            col=0;
        } else {
            col+=1;
        }
    }
    index
}
 fn render(window: &pancurses::Window,world: &Vec<u32>, line_number: u32, x:usize, y:usize, char_map: &Vec<char>, character_char: char) { //Render the map
     window.clear();
     for i in 0..line_number {
         let line = get_line(world.to_vec(), i);
        for n in 0..line.len() {
            if i==y as u32&&n==x as usize{
                window.addch(character_char);
            } else {
                window.addch(char_map[line[n] as usize]);
            }
            
         }
         window.addch('\n');
     }
     }
