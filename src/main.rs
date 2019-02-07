extern crate pancurses;
extern crate serde_json;
use std::io::prelude::*;
use std::fs::File;
extern crate serde;
extern crate dirs;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "sound")]
extern crate ears;
#[cfg(feature = "sound")]
use ears::{Music, Sound, AudioController};
pub mod tools;
pub mod files;
pub mod events;
pub mod gameplay;
use gameplay::game_loop;

#[derive(Serialize, Deserialize, Clone)]
struct UIitem {
    label: String,
    x: i32,
    y: i32,
    item_type: usize,
    start_from_bottom: bool,
    start_from_left: bool,
    selection_id: usize
}


#[derive(Serialize, Deserialize, Clone)]
struct Save {
    map: String,
    x: usize,
    y: usize,
    world: Vec<u32>,
    collision_map: Vec<u8>,
    vars: Vec<i16>
}

use std::path::Path;
use pancurses::{initscr, endwin, noecho};

fn start() {
    //Read game manifest and load initial_map
    
    let mut file = File::open(Path::new(&files::path::get_path("/game/game.json".to_string()))).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not find game config file");
    let config: files::config::Config = serde_json::from_str(&content).unwrap();
        //Start curses mode
        let window = initscr();
        window.refresh();
        window.keypad(true);

        #[cfg(feature = "color")]{
            pancurses::start_color();
            pancurses::use_default_colors();
            let mut colors = 8;
            
            match config.color_mode {
                Some(8)|Some(_)|None => colors = 8,
                Some(16) => colors = 16,
                Some(0) => colors = pancurses::COLORS(),

            }
            let mut i = 0;
            for b in 0..colors {
                window.printw("\n");
                for f in 0..colors {
                    
                    pancurses::init_pair( ( i ) as i16, (f) as i16, (b) as i16);
                    #[cfg(feature = "color_test")] {
                        window.attron(pancurses::ColorPair((i) as u8));
                        window.printw(format!(" ({:?}) ", i));
                        window.attroff(pancurses::ColorPair((i) as u8));
                    }
                    i+=1;
                }
                
            }
            #[cfg(feature = "color_test")] {
                window.getch();
                window.clear();
            }
            
        }
        

        noecho();
        pancurses::curs_set(0);
        gameplay::menu::menu(&window, config.clone());

        
    }



fn main() {
    start();
    endwin();
}


 