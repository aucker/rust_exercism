// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    magazine
        .iter()
        .fold(0, |acc, &word| acc + note.contains(&word) as usize) >= note.len()


    //unimplemented!()
}
