extern crate clap;
use self::clap::{App, Arg, ArgMatches};

pub fn get_args() -> ArgMatches<'static > {
        App::new("legend")
                    .version("0.5 Alpha")
                    .about("A 2D text based game engine")
                    .arg(Arg::with_name("directory")
                            .short("d")
                            .long("directory")
                            .value_name("DIR")
                            .help("Specify a game directory")
                            .takes_value(true))
                    .get_matches()
}