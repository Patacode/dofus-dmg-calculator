use crate::impls;

/// Computes the damage estimation of a Dofus spell.
/// 
/// The spell is considered to inflict `base_damage` damage, by
/// a character having `fixed_damage` fixed damage and `stat_points`
/// stat points allocated to the spell's element, as well as `power`
/// power.
///
/// # Arguments
///
/// * `base_damage` - The spell base damage
/// * `fixed_damage` - The character fixed damage
/// * `stat_points` - The character stat points
/// * `power` - The character power
///
/// # Returns
///
/// The damage estimation of a Dofus spell.
pub fn compute_dmg_estimation(
    base_damage: u64,
    fixed_damage: u64,
    stat_points: u64,
    power: u64,
) -> u64 {
    impls::compute_dmg_estimation(base_damage, fixed_damage, stat_points, power)
}

/// Computes the damage estimation of a Dofus spell inflicted on an enemy with
/// potential resistances.
/// 
/// This function behaves the same as [`compute_dmg_estimation`], but include
/// fixed and variable enemy resistances in the calculation.
///
/// # Arguments
///
/// * `base_damage` - The spell base damage
/// * `fixed_damage` - The character fixed damage
/// * `stat_points` - The character stat points
/// * `power` - The character power
/// * `fixed_res` - The enemy fixed resistance
/// * `variable_res` - The enemy variable resistance
///
/// # Returns
///
/// The damage estimation of a Dofus spell inflicted on an enemy with potential
/// resistances..
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_dmg_estimation_with_null_values() {
        let expected = 0;
        let actual = compute_dmg_estimation(0, 0, 0, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_computes_dmg_estimation_with_positive_values() {
        let expected = 56;
        let actual = compute_dmg_estimation(9, 34, 128, 23);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_computes_dmg_estimation_with_fixed_resistance() {
        let expected = 51;
        let actual = compute_dmg_estimation_with_res(9, 34, 128, 23, 5, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_computes_dmg_estimation_with_variable_resistance() {
        let expected = 45;
        let actual = compute_dmg_estimation_with_res(9, 34, 128, 23, 0, 20);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_computes_dmg_estimation_with_fixed_and_variable_resistance() {
        let expected = 41;
        let actual = compute_dmg_estimation_with_res(9, 34, 128, 23, 5, 20);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_computes_dmg_estimation_with_null_resistances() {
        let expected = 56;
        let actual = compute_dmg_estimation_with_res(9, 34, 128, 23, 0, 0);
        assert_eq!(actual, expected);
    }
}
