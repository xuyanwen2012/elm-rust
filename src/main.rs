extern crate ansi_term;
extern crate clap;

use ansi_term::Colour;
use clap::{App, Arg, SubCommand};

fn main() {
    let _matches = App::new("Featherweight Elm Repl")
        .version("1.0")
        .author("Yanwen Xu. <xuyanwen2012@gmail.com>")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
}
