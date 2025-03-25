pub fn calculate_damage(
    base_damage: u64,
    relevant_stat: u64,
    power: u64,
    fixed_bonus: u64,
) -> u64 {
    let multiplier = 1.0 + (relevant_stat + power) as f64 / 100.0;
    let total = base_damage as f64 * multiplier + fixed_bonus as f64;
    total.floor() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_damage_with_zero_values() {
        let expected = 0;
        let actual = calculate_damage(0, 0, 0, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_damage_with_positive_values() {
        let expected = 56;
        let actual = calculate_damage(9, 128, 23, 34);
        assert_eq!(actual, expected);
    }
}

