// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    /*magazine
        .iter()
        .fold(0, |acc, &word| acc + note.contains(&word) as usize) >= note.len()*/

    let mut mag: HashMap<&str, u32> = HashMap::new();

    magazine
        .iter().for_each(|word| {
        if let Some(val) = mag.get_mut(word) {
            *val += 1
        } else {
            mag.insert(word, 1)
        }
    });

    for word in note.iter() {
        if let Some(val) = mag.get_mut(word) {
            if * val == 1 {
                mag.remove(word)
            } else {
                *val -= 1
            }
        } else {
            return false
        }
    }
    true
    //unimplemented!()
}
