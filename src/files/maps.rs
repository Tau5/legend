use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use files::path;

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub world: Vec<u32>,
    pub collision_map: Vec<u8>,
    pub char_map: Vec<char>,
    pub spawn: [usize; 2],
    pub events: Vec<(String, Vec<(String, String, u32, u32, u32, u32, u32)>)>,
    pub triggers: Vec<(usize, usize, String, usize, usize, usize, usize, usize)>,
    #[cfg(feature = "color")]
    pub char_colors: Vec<u8>
}

pub fn read_worldmap(filename: String) -> World { //Get a World structure from a map file
    let mut file = File::open(Path::new(&path::get_path(format!("{}{}", "/game/", filename).to_string()))).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not find game world file");
    let world_file: World = serde_json::from_str(&content).unwrap();
    world_file
}