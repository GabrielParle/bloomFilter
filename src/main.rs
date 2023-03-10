use std::env;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::io::BufReader;

use fasthash::{farm, metro, mum, spooky};

fn read_file_line_by_line(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut result: Vec<String> = Vec::new();

    for line in reader.lines() {
        result.push(line.unwrap());
    }

    Ok(result)
}

fn hash_to_array(word: &str) -> Vec<u16> {
    let mut result = Vec::new();
    result.push(metro::hash64(word) as u16);
    result.push(farm::hash64(word) as u16);
    result.push(spooky::hash64(word) as u16);
    result.push(mum::hash64(word) as u16);

    result
}

fn flipper(hash_array: Vec<u16>, bool_array: Vec<bool>) {
    for i in hash_array {
        bool_array.get(i)
    }
}

fn add_to_bool_array(bool_array: [bool; 20000000], word: &str) {
    //TODO hash to array func and flipper func
}

fn main() {
    let words = read_file_line_by_line("/usr/share/dict/words").unwrap();
    println!("{}", words.len());
    let mut bool_array = Vec::new();
    for i in 0..u16::MAX {
        bool_array.push(false);
    }

    println!("{:?}", hash_to_array("lemon"))

    //TODO - 4 hash functions COmpostion
    //TODO - 20,000,000 byte array use BITVEC
    //TODO -CLI for suggesting and checking words.
}
