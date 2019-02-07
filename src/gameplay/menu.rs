use files::config::Config;
extern crate pancurses;
use pancurses::Input;
#[cfg(feature = "sound")]
extern crate ears;
#[cfg(feature = "sound")]
use ears::{Sound, AudioController};
use gameplay::saves;
#[cfg(feature = "sound")]
use files;

pub fn menu(window: &pancurses::Window, config: Config){
    let mut selection: usize = config.default_selection;
    let max_selection = config.selection_max; //Temporal, it will be stated by the config in the future
    #[cfg(feature = "sound")]
    let mut select_sound = Sound::new(&files::path::get_path(format!("/game/{}", config.selection_sound))).unwrap();

    render_menu(&window, config.clone(), selection);
    let mut stop = false;
    while !stop  {
        match window.getch() {
            Some(Input::KeyExit)|Some(Input::Character('q')) => {
                break
            },
            Some(Input::KeyDown)|Some(Input::KeyRight)|Some(Input::Character('s'))|Some(Input::Character('d')) => { 
                if selection>0 {
                    selection-=1;
                    #[cfg(feature = "sound")]
                    select_sound.play();
                }
            },
            Some(Input::KeyUp)|Some(Input::KeyLeft)|Some(Input::Character('w'))|Some(Input::Character('a')) => { 
                if selection<max_selection {
                    selection+=1;
                    #[cfg(feature = "sound")]
                    select_sound.play();
                }     
            },
            Some(Input::Character('\n'))|Some(Input::Character('z'))|Some(Input::Character('k')) => { 
                for item in config.clone().ui {
                    if item.selection_id==selection {
                        match item.item_type {
                            0 => stop=true,
                            1 => { saves::new_game(&window, config.clone()); } ,
                            2 => { saves::continue_game(&window, config.clone()); },
                            _ => continue
                        }
                    }
                }
                
            },
            Some(_input) => {continue},
            None => {continue}
        }
        window.clear();
        render_menu(&window, config.clone(), selection);
    }
}

pub fn render_menu(window: &pancurses::Window, config: Config, selection: usize){
    
    for item in config.clone().ui {
        let mut x = item.x;
        let mut y = item.y;
        if item.start_from_bottom {
            y = window.get_max_y() - item.y;
        }
        if item.start_from_left {
            x = window.get_max_x() - item.x;
        }
        window.mv(y, x);
        match item.item_type {
            0 => {
                if selection == item.selection_id { 
                    window.printw(format!("* {}", item.label)); 

                } else { window.printw(item.label); }
                
            },
            1 => {
                if selection == item.selection_id { 
                    window.printw(format!("* {}", item.label)); 
                    
                } else { window.printw(item.label); }
                
            },
            2 => {
                if selection == item.selection_id { window.printw(format!("* {}", item.label)); } else { window.printw(item.label); }
            },
            3 => {
                if item.label != "" { window.printw(item.label); } else { window.printw(config.clone().name); }
            }
            _ => {continue}
        }
    }


    /*
    UI Item Type table

    --------------------------------------------------------------------------------
    | ID | Name    | Selectable? | Description                                     |
    --------------------------------------------------------------------------------
    |0   |Quit     | Yes         | Button that quits the game                      |
    |1   |New Game | Yes         | Button to start a new game                      |
    |2   |Continue | Yes         | Button to continue the game from the save file  |
    |3   |Title    | No          | A text label that displays the name of the game | 
    |                              if a label is provided it will be displayed     |
    --------------------------------------------------------------------------------
    */
}
