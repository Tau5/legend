
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use files::path;
    #[derive(Serialize, Deserialize, Clone)]
    pub struct UIitem {
        pub label: String,
        pub x: i32,
        pub y: i32,
        pub item_type: usize,
        pub start_from_bottom: bool,
        pub start_from_left: bool,
        pub selection_id: usize
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Config {
        pub name: String,
        pub author: String,
        pub short_name: String,
        pub initial_map: String,
        #[cfg(feature = "sound")]
        pub selection_sound: String,
        #[cfg(feature = "sound")]
        pub interact_sound: String,
        pub ui: Vec<UIitem>,
        pub selection_max: usize,
        pub default_selection: usize,
        pub color_mode: Option<u8>
    }
    pub fn get_config() -> Config {
        let mut file = File::open(Path::new(&path::get_path("/game.json".to_string()))).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Could not find game config file");
        let config: Config = serde_json::from_str(&content).unwrap();
        config
    }