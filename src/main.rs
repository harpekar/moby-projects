use std::path::Path;
use serde::{Deserialize};
use serde_json::Result;
use rand::thread_rng;
use std::fmt;

#[derive(Deserialize, Debug)]
struct Word {
    //word: String,
    pos: String,
}

impl fmt::Display for Word {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({})", self.pos)
    }

}

fn import_words<P: AsRef<Path>>(path: P) -> Result<Vec<Word>> {

    let json_file: String = std::fs::read_to_string(path).unwrap();

    println!("{}", json_file);

    let words: Vec<Word> = serde_json::from_str(&json_file).unwrap();
    
    Ok(words)

}

fn main() {
    
    let rng = thread_rng();

    let words = import_words("../mobywords.json");

    for word in words.iter() {
        println!("{:?}", word);
    }

    //let adj = words.retain(|&word| word.pos == "A");

    //println!("{} spice", adj.choose(&mut rng)); 
    
}
