use crate::{character::Character, enemy::Enemy};

pub mod battle_action;
pub mod damage;
pub mod skill;

pub struct BattleState {
    pub enemy: Enemy,
    pub characters: Vec<Character>,

    pub log_text: Vec<String>,
}
