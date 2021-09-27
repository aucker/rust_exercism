// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]


use std::thread::require_unstable_const_init_thread_local;

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
            mana,
            level: self.level
        })



        //unimplemented!("Revive this player")
    }

    //not a wizard: health - spell_cost    no damage
    //wizard but not enough mana: no damage and not mana consume
    //wizard: mana consume and damage 2 time perform
    //damage with enough mana pool
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            if self.health < mana_cost {
                self.health = 0
            } else {
                self.health = self.health - mana_cost
            }
            return 0
        }

        let mut cur_mana = self.mana.unwrap();
        if cur_mana < mana_cost {
            return 0
        }
        cur_mana = cur_mana - mana_cost;
        self.mana.replace(cur_mana);
        return mana_cost * 2
        //unimplemented!("Cast a spell of cost {}", mana_cost)
    }
}
