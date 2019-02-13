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

use std::path::Path;
use pancurses::{initscr, endwin, noecho};

fn start() {
    //Read game manifest and load initial_map
    
    let config = files::config::get_config();
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


 