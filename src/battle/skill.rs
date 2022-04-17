use super::battle_action::SkillFn;

pub struct Skill {
    pub curr_cooldown: u32,
    pub data: &'static SkillData,
}

impl Skill {
    pub fn from_skilldata(data: Option<&'static SkillData>) -> Option<Skill> {
        data.and_then(|data| {
            Some(Skill {
                curr_cooldown: 0,
                data,
            })
        })
    }
}

pub struct SkillData {
    pub name: &'static str,
    pub description: &'static str,
    pub cooldown: u32,

    pub function: SkillFn,
}
