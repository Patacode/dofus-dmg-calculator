pub fn calculate_damage(base_damage: u64, relevant_stat: u64, power: u64, fixed_bonus: u64) -> u64 {
    let multiplier = 1.0 + (relevant_stat + power) as f64 / 100.0;
    let total = base_damage as f64 * multiplier + fixed_bonus as f64;
    total.floor() as u64
}
