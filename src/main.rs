use std::path::Path;
use serde::{Deserialize};
use serde_json::{Result, Value};
use rand::{thread_rng, seq::SliceRandom};
use std::fmt;

#[derive(Deserialize, Debug)]
struct SpeechPart {
    pos: String,
    words: Vec<Value>          
}

impl fmt::Display for SpeechPart {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({})", self.pos)
    }

}

fn import_words<P: AsRef<Path>>(path: P) -> Result<Value> {

    let json_file: String = std::fs::read_to_string(path).unwrap();

    let words: Value = serde_json::from_str(&json_file).unwrap();
    
    Ok(words)

}

fn main() {
    
    let mut rng = thread_rng();

    let words = import_words("../mobywords.json").unwrap();
        
    let adjectives = words.get("Adjective").unwrap().as_array().unwrap();


    println!("{} spice", adjectives.choose(&mut rng).unwrap().as_str().unwrap());

    //println!("{}", words["ambiguous"]);

    //println!("{} spice", adj.choose(&mut rng)); 
    
}
