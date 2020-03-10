extern crate ansi_term;
extern crate clap;

use ansi_term::Colour;
use clap::{App, Arg, SubCommand};
use std::io::{self, Write};

// Workspace
use rustelm_analyzer::typechecker;
use rustelm_parser::parser;

/// Prompt input from user, return the string
fn input() -> io::Result<String> {
    io::stdout().write("> ".as_bytes()).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn eval(mut input: String) -> Result<String, &'static str> {
    input.push('\n');

    match parser::parse(input.as_str()) {
        Ok(expr) => match typechecker::typecheck_root(expr) {
            Ok(ty) => Ok(format!("{:?}", ty)),
            Err(_) => Err("Typecheck Error"),
        },
        Err(_) => Err("Parse Error"),
    }
}

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
        "{} {} {}\n{}\n{}",
        Colour::RGB(128, 128, 128).paint("----"),
        Colour::Green.paint("FElm 0.1.0"),
        Colour::RGB(128, 128, 128)
            .paint("-----------------------------------------------------------"),
        Colour::RGB(128, 128, 128)
            .paint("Read <https://github.com/xuyanwen2012/elm-rust> to learn about the language"),
        Colour::RGB(128, 128, 128)
            .paint("---------------------------------------------------------------------------"),
    );

    loop {
        let source = input().unwrap();

        // Eval
        match eval(source) {
            Ok(ty_str) => println!("{:?}", ty_str),
            Err(e) => println!("{:?}", e),
        }
    }
}
