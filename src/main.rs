use std::path::Path;
use std::io::{Write, stdout, stdin};
use serde_json::{Result, Value};
use rand::{thread_rng, seq::SliceRandom};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

fn import_words<P: AsRef<Path>>(path: P) -> Result<Value> {

    let json_file: String = std::fs::read_to_string(path).unwrap();

    let words: Value = serde_json::from_str(&json_file).unwrap();
    
    Ok(words)

}

fn print_girl(adj: &Vec<Value>) {

    let mut rng = thread_rng();
    
    println!("{} Spice \r", adj.choose(&mut rng).unwrap().as_str().unwrap());

}

fn main() {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    println!("Welcome to Spice Girl Generator, press escape to exit and any other key to generate a new name. \r");

    stdout.flush().unwrap();

    let words = import_words("../mobywords.json").unwrap();
        
    let adjectives = words.get("Adjective").unwrap().as_array().unwrap();

    for c in stdin.keys() {

        write!(
            stdout,
            "{}",
            //termion::cursor::Goto(1, 1),
            termion::clear::UntilNewline
        )
        .unwrap();

        match c.unwrap() {

            Key::Esc => break,
            _ => print_girl(adjectives), 

        }
    }
    
}
