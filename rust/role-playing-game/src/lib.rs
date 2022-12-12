use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            None
        } else {
            let mana = if self.level < 10 { None } else { Some(100) };

            Some(Player {
                health: 100,
                mana,
                level: self.level,
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(ref mut current_mana) if *current_mana >= mana_cost => {
                *current_mana -= mana_cost;
                mana_cost * 2
            }
            Some(_) => 0,
            None => {
                self.health -= min(mana_cost, self.health);
                0
            }
        }
    }
}
