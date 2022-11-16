use ansi_term::Colour::{Red};
use crate::DEBUG;

#[derive(Debug, Clone, Copy)]
pub struct KnownLetter {
    pub letter: char,
    pub pos: usize,
}

impl Default for KnownLetter {
    fn default() -> Self {
        KnownLetter { letter: '_', pos: 0 }
    }
}

impl KnownLetter {
    // Increment position of some KnownLetter
    pub fn increment_knowns(&mut self) {
        self.pos += 1;
    }
}

// Take a "A_B__C____D___" formatted string and produce a vector of KnownLetters
pub fn populate_knowns(string: &str) -> Vec<KnownLetter> {
    let mut knowns: Vec<KnownLetter> = Vec::new();
    let mut i: usize = 0;
    for (position, character) in string.char_indices() {
        if character != '_' {
            knowns.push(KnownLetter {letter: character, pos: position});
        }
        i = i + 1;
    }
    if DEBUG > 0 {
        println!("{}", Red.underline().paint("@populate_knowns"));
        println!("{:?}", knowns);
    }

    knowns
}

// Remove KnownLetters from key to give relevant letters to permute
pub fn remove_knowns(pzl_key: &mut String, knowns: &Vec<KnownLetter>) {
    for value in knowns {
        pzl_key.remove(pzl_key.find(value.letter)
            .expect(format!("Could not find letter '{}' in key '{}'", value.letter, pzl_key).as_str()));
    }
}

// Insert KnownLetters into a string at their intended index
pub fn restore_knowns(s: &mut String, known_letters: &Vec<KnownLetter>) {
    for j in known_letters {
        s.insert(j.pos, j.letter);
    }
}

// Generate STRING of KnownLetter in format "A_B___C__D____"
//  uses restore_knowns() but with more functionality
pub fn generate_knowns(bank: &str, knowns: Option<&Vec<KnownLetter>>) -> String {
    let mut knowns_str: String = String::new();
    let length: usize = bank.chars().count();
    // in default case, make empty vector and generate empty "___+n" string
    let empty = vec![KnownLetter::default()];
    let tmp_knowns: &Vec<KnownLetter> = knowns.unwrap_or(&empty);
    let knowns_len: usize = knowns.unwrap_or(&vec![]).capacity();
    for i in 0..(length - knowns_len) {
        knowns_str.insert(i, '_');
    }

    restore_knowns(&mut knowns_str, tmp_knowns);
    if DEBUG > 0 {
        println!("{}", Red.underline().paint("@generate_knowns"));
        println!("[{}] :: {}", Red.underline().paint(&knowns_str), knowns_str.chars().count());
    }

    knowns_str
}


pub fn run_stride(string: &str, stride: &str, actual_knowns: &str) -> Vec<String>{
    let mut current_knowns = populate_knowns(stride);
    let mut knowns_str: String;
    let mut str_set: Vec<String> = Vec::new();
    str_set.push(generate_knowns(string, Some(&current_knowns)));
    
    let top = string.chars().count() - stride.chars().count() - 1;
    for _ in 0..top - 1 {
        for k in current_knowns.as_mut_slice() {
            if k.pos < string.chars().count() {
                k.increment_knowns();
            } else {}
        }
        knowns_str = generate_knowns(string, Some(&current_knowns));
        str_set.push(knowns_str);
    }

    str_set
}
