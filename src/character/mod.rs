use crate::battle::{
    battle_action::ChargedAtkFn,
    skill::{Skill, SkillData},
};

pub struct Character {
    pub hp: f64,
    pub max_hp: f64,
    pub atk: f64,
    // def???

    // buffs
    pub skill1: Option<Skill>,
    pub skill2: Option<Skill>,
    pub skill3: Option<Skill>,
    pub skill4: Option<Skill>,

    pub data: &'static CharacterData,
}

impl Character {
    pub fn new(data: &'static CharacterData) -> Self {
        Self {
            hp: data.max_hp,
            max_hp: data.max_hp,
            atk: data.max_atk,
            skill1: Skill::from_skilldata(data.skill1),
            skill2: Skill::from_skilldata(data.skill2),
            skill3: Skill::from_skilldata(data.skill3),
            skill4: Skill::from_skilldata(data.skill4),
            data,
        }
    }

    pub fn charge_atk(&self) -> ChargedAtkFn {
        self.data.charge_atk
    }

    pub fn get_skill(&self, n: usize) -> Option<&Skill> {
        match n {
            0 => self.skill1.as_ref(),
            1 => self.skill2.as_ref(),
            2 => self.skill3.as_ref(),
            3 => self.skill4.as_ref(),
            _ => None,
        }
    }

    pub fn get_skill_mut(&mut self, n: usize) -> Option<&mut Skill> {
        match n {
            0 => self.skill1.as_mut(),
            1 => self.skill2.as_mut(),
            2 => self.skill3.as_mut(),
            3 => self.skill4.as_mut(),
            _ => None,
        }
    }
}

pub struct CharacterData {
    pub name: &'static str,
    pub element: Element,
    max_hp: f64,
    max_atk: f64,

    charge_atk: ChargedAtkFn,
    skill1: Option<&'static SkillData>,
    skill2: Option<&'static SkillData>,
    skill3: Option<&'static SkillData>,
    skill4: Option<&'static SkillData>,

    pub skill_count: usize,
}

pub enum Element {
    Light,
    Dark,
    Fire,
    Water,
    Earth,
    Wind,
}

pub mod katalina_grand;
