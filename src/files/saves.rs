use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use files::config;
    #[derive(Serialize, Deserialize, Clone)]
    pub struct Save {
        pub map: String,
        pub x: usize,
        pub y: usize,
        pub world: Vec<u32>,
        pub collision_map: Vec<u8>,
        pub vars: Vec<i16>
    }

    pub fn has_save() -> bool {
        Path::new(&get_save_path()).exists()
    }
    /*
    fn get_save_name() -> String {
        format!("{}.save", get_config().clone().short_name)
    }
    */
    pub fn get_save_path() -> String {
        Path::new(&format!("{}/legend/saves/{}.save", dirs::home_dir().unwrap().display(), config::get_config().clone().short_name)).display().to_string()
    }
    pub fn get_save() -> Save {
        let mut file = File::open(Path::new(&format!("{}/legend/saves/{}.save", dirs::home_dir().unwrap().display(), config::get_config().clone().short_name))).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Some error ocurred and your save file could not be loaded");
        let save: Save = serde_json::from_str(&content).unwrap();
        save
    }   

