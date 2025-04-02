use crate::impls;

pub fn compute_dmg_estimation(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
) -> u64 {
    impls::compute_dmg_estimation(base_damage, fixed_damage, stat_points, power)
}

pub fn compute_dmg_estimation_with_res(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
    fixed_res: u64,
    variable_res: u64,
) -> u64 {
    impls::compute_dmg_estimation_with_res(
        base_damage,
        fixed_damage,
        stat_points,
        power,
        fixed_res,
        variable_res,
    )
}
