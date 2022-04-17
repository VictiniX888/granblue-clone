use crate::battle::{
    damage::{charged_atk_damage, skill_damage},
    BattleState,
};

use super::{CharacterData, Element, SkillData};

pub const KATALINA_GRAND: CharacterData = CharacterData {
    name: "Katalina",
    element: Element::Water,
    max_hp: 2310_f64,
    max_atk: 9450_f64,
    charge_atk,
    skill1: Some(&SKILL_1),
    skill2: Some(&SKILL_2),
    skill3: Some(&SKILL_3),
    skill4: None,
    skill_count: 3,
};

fn charge_atk(state: &mut BattleState, char_i: usize) -> String {
    let damage = charged_atk_damage(state.characters[char_i].atk, state.enemy.def, 4.5);
    state.damage_enemy(damage);

    format!(
        "{} used Charged Attack! Dealt {} damage!",
        state.characters[char_i].data.name, damage
    )
}

const SKILL_1: SkillData = SkillData {
    name: "Enchanted Lands",
    description: "300% Water damage to a foe.",
    cooldown: 6,
    function: skill1,
};

fn skill1(state: &mut BattleState, char_i: usize) -> String {
    let damage = skill_damage(state.characters[char_i].atk, state.enemy.def, 3.0);
    state.damage_enemy(damage);

    format!(
        "{} used {}! Dealt {} damage!",
        state.characters[char_i].data.name,
        state.characters[char_i]
            .data
            .skill1
            .and_then(|skill| Some(skill.name))
            .unwrap_or("ERROR"),
        damage
    )
}

const SKILL_2: SkillData = SkillData {
    name: "Loengard",
    description: "400% Water damage to a foe.",
    cooldown: 8,
    function: skill2,
};

fn skill2(state: &mut BattleState, char_i: usize) -> String {
    let damage = skill_damage(state.characters[char_i].atk, state.enemy.def, 4.0);
    state.damage_enemy(damage);

    format!(
        "{} used {}! Dealt {} damage!",
        state.characters[char_i].data.name,
        state.characters[char_i]
            .data
            .skill1
            .and_then(|skill| Some(skill.name))
            .unwrap_or("ERROR"),
        damage
    )
}

const SKILL_3: SkillData = SkillData {
    name: "Light Wall Divider",
    description: "500% Water damage to a foe.",
    cooldown: 3,
    function: skill3,
};

fn skill3(state: &mut BattleState, char_i: usize) -> String {
    let damage = skill_damage(state.characters[char_i].atk, state.enemy.def, 5.0);
    state.damage_enemy(damage);

    format!(
        "{} used {}! Dealt {} damage!",
        state.characters[char_i].data.name,
        state.characters[char_i]
            .data
            .skill1
            .and_then(|skill| Some(skill.name))
            .unwrap_or("ERROR"),
        damage
    )
}
