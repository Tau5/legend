use files::maps::World;
use tools;
#[allow(unused_variables)]

pub fn render(window: &pancurses::Window,world: &Vec<u32>, line_number: u32, x:usize, y:usize, char_map: &Vec<char>, character_char: char, message: String, world_file: World) { //Render the map
     window.clear();
     for i in 0..line_number {
         let line = tools::get_line(world.to_vec(), i);
        for n in 0..line.len() {
            if i==y as u32&&n==x as usize{
                #[cfg(feature = "color")]
                window.attron(pancurses::ColorPair(world_file.char_colors[line[n] as usize] ));
                window.addch(character_char);
                #[cfg(feature = "color")]
                window.attroff(pancurses::ColorPair( world_file.char_colors[line[n] as usize] ));
            } else {
                
                if cfg!(feature = "color") {
                    #[cfg(feature = "color")]
                    window.attron(pancurses::ColorPair(world_file.char_colors[line[n] as usize]));
                    window.addch(char_map[line[n] as usize]);
                    #[cfg(feature = "color")]
                    window.attroff(pancurses::ColorPair(world_file.char_colors[line[n] as usize]));
                } else {
                    window.addch(char_map[line[n] as usize]);
                }
            }
            
         }
        window.addch('\n');
     }
        window.printw("\n\n");
        window.printw(message);
}
