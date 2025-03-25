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
}
