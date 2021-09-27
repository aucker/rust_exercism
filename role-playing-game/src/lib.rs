// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::slice::heapsort;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }
        let mut mana: Option<u32> = None;
        if self.level >= 10 {
            mana = Some(100);
        }
        return Some(Player {
            health: 100,
            mana: mana,
            level: self.level
        })



        //unimplemented!("Revive this player")
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        unimplemented!("Cast a spell of cost {}", mana_cost)
    }
}
