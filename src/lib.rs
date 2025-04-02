mod api;
mod impls;

pub use api::compute_dmg_estimation;
pub use api::compute_dmg_estimation_with_res;

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
