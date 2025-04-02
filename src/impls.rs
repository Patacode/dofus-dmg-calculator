pub fn compute_dmg_estimation(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
) -> u64 {
    let multiplier = 1.0 + (stat_points + power) as f64 / 100.0;
    let total = base_damage as f64 * multiplier + fixed_damage as f64;
    total.floor() as u64
}

pub fn compute_dmg_estimation_with_res(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
    fixed_res: u64,
    variable_res: u64,
) -> u64 {
    let multiplier = 1.0 + (stat_points + power) as f64 / 100.0;
    let total = base_damage as f64 * multiplier + fixed_damage as f64;
    let linear_reduction = total - fixed_res as f64;
    let final_res = linear_reduction * (1.0 - variable_res as f64 / 100.0);
    final_res.floor() as u64
}
