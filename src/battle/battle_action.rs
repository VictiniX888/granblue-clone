use super::{damage::normal_damage, BattleState};

pub type SkillFn = fn(&mut BattleState, usize) -> String;
pub type ChargedAtkFn = fn(&mut BattleState, usize) -> String;

pub enum BattleAction {
    DamageEnemy { damage: f64 },
    ResetSkillCooldown { character_i: usize, skill_n: usize },
}

// public impl
impl BattleState {
    pub fn skill(&mut self, n: usize, char_i: usize) {
        if let Some(skill) = self.characters[char_i].get_skill(n) {
            if skill.curr_cooldown == 0 {
                let log = (skill.data.function)(self, char_i);
                self.log_text.push(log);

                self.reset_skill_cooldown(char_i, n);
            }
        }
    }

    pub fn attack(&mut self) {
        for char_i in 0..self.characters.len() {
            let damage = normal_damage(self.characters[char_i].atk, self.enemy.def);
            self.damage_enemy(damage);
            self.log_text.push(format!(
                "{} dealt {} damage!",
                self.characters[char_i].data.name, damage
            ));

            for skill_n in 0..self.characters[char_i].data.skill_count {
                self.decrement_skill_cooldown(char_i, skill_n);
            }
        }
    }
}

// "private" impl
impl BattleState {
    pub(crate) fn damage_enemy(&mut self, damage: f64) {
        self.enemy.decrease_hp(damage);
    }

    pub(crate) fn reset_skill_cooldown(&mut self, char_i: usize, skill_n: usize) {
        if let Some(skill) = self.characters[char_i].get_skill_mut(skill_n) {
            skill.curr_cooldown = skill.data.cooldown;
        };
    }

    pub(crate) fn decrement_skill_cooldown(&mut self, char_i: usize, skill_n: usize) {
        if let Some(skill) = self.characters[char_i].get_skill_mut(skill_n) {
            if skill.curr_cooldown > 0 {
                skill.curr_cooldown -= 1;
            }
        };
    }
}
