fn base_damage(atk: f64 /* other boosts */) -> f64 {
    atk /* times other boosts */
}

pub fn normal_damage(atk: f64, enemy_def: f64 /* other boosts */) -> f64 {
    base_damage(atk) / enemy_def /* random modifier */
}

pub fn charged_atk_damage(
    atk: f64,
    enemy_def: f64,
    multiplier: f64, /* other boosts */
) -> f64 {
    normal_damage(atk, enemy_def) * multiplier /* other boosts */
}

pub fn skill_damage(atk: f64, enemy_def: f64, multiplier: f64 /* other boosts */) -> f64 {
    normal_damage(atk, enemy_def) * multiplier /* other boosts */
}
