extern crate pancurses;
fn main() {
        let window = initscr();
        window.refresh();
        window.keypad(true);
        pancurses::start_color();
        pancurses::use_default_colors();
        let mut i = 0;
        let mut colors = 8;
        for b in 0..colors {
            window.printw("\n");
            for f in 0..colors {
                pancurses::init_pair( ( i ) as i16, (f) as i16, (b) as i16);
                window.attron(pancurses::ColorPair((i) as u8));
                window.printw(format!(" {:?}-{:?} ", b, f));
                window.attroff(pancurses::ColorPair((i) as u8));
                i+=1;
            }
        }
}