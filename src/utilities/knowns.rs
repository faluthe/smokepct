#[derive(Debug, Clone, Copy)]
pub struct KnownLetter {
    pub letter: char,
    pub pos: usize,
}

pub fn populate_knowns(string: &str) -> Vec<KnownLetter> {
    let mut knowns: Vec<KnownLetter> = Vec::new();
    let mut i: usize = 0;
    for (position, character) in string.char_indices() {

        if character != '_' {
            knowns.push(KnownLetter {letter: character, pos: position});
        }
        i = i + 1;
    }
    // println!("{:?}", knowns);
    knowns
}

pub fn remove_knowns(pzl_key: &mut String, knowns: Vec<KnownLetter>) 
        -> Option<&mut String> {
    // key.remove(key.find(letter)?);
    for value in knowns {
        pzl_key.remove(pzl_key.find(value.letter)?);
    }
    Some(pzl_key)
}

pub fn restore_knowns(s: &mut String, k: &Vec<KnownLetter>) {
    for j in k {
        s.insert(j.pos, j.letter);
    }
}
