use files::maps::World;
use files::config::Config;
use events::triggers;
extern crate pancurses;
use pancurses::Input;
use tools;
use files;
use std::io::prelude::*;
use std::fs::File;
use std::io;
#[cfg(feature = "sound")]
extern crate ears;
#[cfg(feature = "sound")]
use ears::{Sound, AudioController};
pub mod renderization;
pub mod menu;
pub mod saves;

pub fn game_loop(world_file: World, window: &pancurses::Window, mut world:Vec<u32>, char_map: Vec<char>, mut collision_map:Vec<u8>, cus_coor: bool, cus_x: usize, cus_y: usize, actual_map: String, config: Config, vars: &mut Vec<i16>) {
    #[cfg(feature = "sound")]
    let mut interact_sound = Sound::new(&files::path::get_path(format!("/game/{}", config.interact_sound))).unwrap();
    let mut message: String = "".to_string();
    let mut x;
    let mut y;
    if cus_coor {
        x = cus_x;
        y = cus_y;
    } else {
        x = world_file.spawn[0];
        y = world_file.spawn[1];
    }
    let mut facing: u8 = 1;
    let trigger_data = triggers::check_init_triggers(&window,&world_file,&mut world, &mut collision_map, x, y, config.clone(), vars); //Read for triggers
    if trigger_data.0==1 { return ; }
    if trigger_data.1 != "" {
            message = trigger_data.clone().1;
    }
    let trigger_data = triggers::check_triggers(&window,&world_file,&mut world, &mut collision_map, x, y, config.clone(), vars); //Read for triggers
    if trigger_data.0==1 { return ; }
    if trigger_data.1 != "" {
            message = trigger_data.clone().1;
    }
    
    renderization::render(&window, &world, tools::get_line_count(&world), x, y, &char_map, '*', message.clone(), world_file.clone()); //Render the map
    loop {
        match window.getch() {
            Some(Input::KeyLeft)|Some(Input::Character('a')) => { 
                facing = 3;
                if x>0&&tools::check_collision(x-1, y, &collision_map) {
                    x-=1;
                }
            },
            Some(Input::KeyRight)|Some(Input::Character('d')) => { 
                facing = 1;
                if tools::check_collision(x+1, y, &collision_map) {
                    x+=1;
                } 
            },
            Some(Input::KeyUp)|Some(Input::Character('w')) => { 
                facing = 0;
                if y>0&&tools::check_collision(x, y-1, &collision_map) {
                    y-=1;
                }
            },
            Some(Input::KeyDown)|Some(Input::Character('s')) => { 
                facing = 2;
                if tools::check_collision(x, y+1, &collision_map) {
                    y+=1;
                }
                    
            },
            Some(Input::KeyExit)|Some(Input::Character('q')) => {
                let save = files::saves::Save {
                    map: actual_map.clone(),
                    x: x,
                    y: y,
                    world: world.clone(),
                    collision_map: collision_map.clone(),
                    vars: vars.to_vec()
                };
                
                let save_str = serde_json::to_string(&save).unwrap();
                if files::saves::has_save() {
                    files::path::check_legend_dirs();
                    let file = File::create(&format!("{}/legend/saves/{}.save", dirs::home_dir().unwrap().display(), config.clone().short_name)).unwrap();
                    let mut writer = io::BufWriter::new(&file);
                    write!(writer, "{}", save_str);
                    
                } else {
                    files::path::check_legend_dirs();
                    let file = File::create(&format!("{}/legend/saves/{}.save", dirs::home_dir().unwrap().display(), config.clone().short_name)).unwrap();
                    let mut writer = io::BufWriter::new(&file);
                    write!(writer, "{}", save_str);
                }
                break
            },
            Some(Input::Character('k'))|Some(Input::Character('z')) => {
                #[cfg(feature = "sound")]
                interact_sound.play();

                let trigger_data = triggers::check_interactable_triggers(&window, &world_file ,&mut world, &mut collision_map, x, y, facing, config.clone(), vars); //Read for interact triggers
                if trigger_data.0==1 { break ; }
                if trigger_data.1 != "" {
                            message = trigger_data.clone().1;
                }
            },
            Some(Input::Character('r')) => {
                let save = files::saves::Save {
                    map: actual_map.clone(),
                    x: x,
                    y: y,
                    world: world.clone(),
                    collision_map: collision_map.clone(),
                    vars: vars.to_vec()
                };
                
                let save_str = serde_json::to_string(&save).unwrap();
                if files::saves::has_save() {
                    files::path::check_legend_dirs();
                    let file = File::create(&format!("{}/legend/saves/{}.save", dirs::home_dir().unwrap().display(), config.clone().short_name)).unwrap();
                    let mut writer = io::BufWriter::new(&file);
                    write!(writer, "{}", save_str);
                    
                } else {
                    files::path::check_legend_dirs();
                    let file = File::create(&format!("{}/legend/saves/{}.save", dirs::home_dir().unwrap().display(), config.clone().short_name)).unwrap();
                    let mut writer = io::BufWriter::new(&file);
                    write!(writer, "{}", save_str);
                }
            },
            Some(_input) => {continue},
            None => {continue}
        }
        window.clear();
        let trigger_data = triggers::check_triggers(&window, &world_file.clone() ,&mut world, &mut collision_map, x, y, config.clone(), vars); //Read for triggers
        if trigger_data.0==1 { break ; }
        if trigger_data.1 != "" {
            message = trigger_data.clone().1;
        }
        
        renderization::render(&window, &world, tools::get_line_count(&world), x, y, &char_map, '*', message.clone(), world_file.clone());
        
    }
    
}
