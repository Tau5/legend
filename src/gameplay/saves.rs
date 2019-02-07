use files::config::Config;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use files;
use files::maps::World;
use gameplay::game_loop;
pub fn new_game(window: &pancurses::Window, config: Config){
    let mut vars: Vec<i16> = Vec::new();
    let actual_map = config.clone().initial_map;
    let mut file;
    let mut content = String::new();
    
    file = File::open(Path::new(&files::path::get_path(format!("{}{}", "/game/", config.initial_map).to_string()))).unwrap();
    file.read_to_string(&mut content).expect("Could not find game world file");
    let world_file: World = serde_json::from_str(&content).unwrap();
    let world = world_file.world.clone();
    let char_map = world_file.char_map.clone();
    let collision_map = world_file.collision_map.clone();
    game_loop(world_file.clone(), &window, world, char_map, collision_map, false, 0,0, actual_map, config.clone(), &mut vars);
}
pub fn continue_game(window: &pancurses::Window, config: Config){
    if !files::saves::has_save() { new_game(&window, config.clone()); return; }
    let save = files::saves::get_save();
    let actual_map = save.clone().map;
    let mut file;
    let mut content = String::new();
    
    file = File::open(Path::new(&files::path::get_path(format!("{}{}", "/game/", save.clone().map).to_string()))).unwrap();
    file.read_to_string(&mut content).expect("Could not find game world file");
    let world_file: World = serde_json::from_str(&content).unwrap();
    let char_map = world_file.char_map.clone();
    let mut vars = save.clone().vars;
    game_loop(world_file.clone(), &window, save.world, char_map, save.collision_map, true, save.x, save.y, actual_map, config.clone(), &mut vars);
}