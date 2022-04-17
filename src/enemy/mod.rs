pub struct Enemy {
    pub name: &'static str,

    pub hp: f64,
    pub max_hp: f64,

    pub def: f64,
}

impl Enemy {
    pub fn decrease_hp(&mut self, dec: f64) {
        if self.hp >= dec {
            self.hp -= dec;
        } else {
            self.hp = 0_f64;
        }
    }
}

pub struct EnemyData {}
