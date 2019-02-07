use events;
use files::maps::World;
use files::config::Config;
pub fn check_interactable_triggers(window: &pancurses::Window, world:&World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize, facing: u8, config: Config, vars: &mut Vec<i16>) -> (u8, String) { 
    let mut return_data: (u8, String) = (0,"".to_string());
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
            return_data = events::run_event(i.clone().2, window, &world, world_map, collision_map, x, y, config.clone(), vars);
        }
    }
    return_data
}
pub fn check_triggers(window: &pancurses::Window, world:&World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize, config: Config, vars: &mut Vec<i16>) -> (u8, String) { 
    let mut return_data: (u8, String) = (0,"".to_string());
    for i in world.triggers.iter() { //Iterate trough triggers to check if a events must be ran
        if i.0==x&&i.1==y&&i.3==0 {
            return_data = events::run_event(i.clone().2, window, &world, world_map, collision_map, x, y, config.clone(), vars);
        }
    }
    return_data
}
pub fn check_init_triggers(window: &pancurses::Window, world:&World, world_map: &mut Vec<u32>, collision_map: &mut Vec<u8>, x:usize, y:usize, config: Config, vars: &mut Vec<i16>) -> (u8, String) { 
    let mut return_data: (u8, String) = (0,"".to_string());
    for i in world.triggers.iter() { //Iterate trough triggers to check if a events must be ran
        if i.3==2 {
            return_data = events::run_event(i.clone().2, window, &world, world_map, collision_map, x, y, config.clone(), vars);
        }
    }
    return_data
}