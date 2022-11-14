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

// Take a "A_B__C____D___" formatted string and produce a vector of KnownLetters
pub fn populate_knowns(string: Option<&str>) -> Vec<KnownLetter> {
    let mut knowns: Vec<KnownLetter> = Vec::new();
    let mut i: usize = 0;
    for (position, character) in string.unwrap_or_default().char_indices() {

        if character != '_' {
            knowns.push(KnownLetter {letter: character, pos: position});
        }
        i = i + 1;
    }
    if DEBUG > 0 {
        println!("@populate_knowns");
        println!("{:?}", knowns);
    }

    knowns
}

// Remove KnownLetters from key to give relevant letters to permute
pub fn remove_knowns(pzl_key: &mut String, knowns: Vec<KnownLetter>) 
        -> Option<&mut String> {
    // key.remove(key.find(letter)?);
    for value in knowns {
        pzl_key.remove(pzl_key.find(value.letter)?);
    }
    Some(pzl_key)
}

// Insert KnownLetters into a string at their intended index
pub fn restore_knowns(s: &mut String, k: &Vec<KnownLetter>) {
    for j in k {
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
        println!("@generate_knowns");
        println!("[{}] :: {}", knowns_str, knowns_str.chars().count());
    }

    knowns_str
}
// Increment position of some KnownLetter
pub fn increment_knowns(known: &KnownLetter) -> KnownLetter {
    let new = known.pos + 1;

    return KnownLetter { letter: known.letter, pos: new }
}
