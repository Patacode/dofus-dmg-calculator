pub fn compute_dmg_estimation(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
) -> u64 {
    let total = compute_dmg_estimation_no_floor(
        base_damage,
        fixed_damage,
        stat_points,
        power,
    );

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
    let total = compute_dmg_estimation_no_floor(
        base_damage,
        fixed_damage,
        stat_points,
        power,
    );
    let linear_reduction = total - fixed_res as f64;
    let final_res = linear_reduction * (1.0 - variable_res as f64 / 100.0);
    final_res.floor() as u64
}

fn compute_dmg_estimation_no_floor(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
) -> f64 {
    let multiplier = 1.0 + (stat_points + power) as f64 / 100.0;
    base_damage as f64 * multiplier + fixed_damage as f64
}
