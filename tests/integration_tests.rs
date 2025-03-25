use test_bin::get_test_bin;

#[test]
fn it_outputs_correct_damage_estimation_with_no_args() {
    let mut cmd = get_test_bin("dofus-dmg-calculator");
    let output = cmd.output().expect("Failed to execute command");
    let expected = "Damage estimation = 0 - 0 (0 - 0)\n";
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert_eq!(expected, actual);
}

#[test]
fn it_outputs_correct_damage_estimation_with_all_args() {
    let mut cmd = get_test_bin("dofus-dmg-calculator");

    cmd
        .args(["-i", "9"])
        .args(["-j", "11"])
        .args(["-k", "12"])
        .args(["-l", "14"])
        .args(["-s", "128"])
        .args(["-p", "12"])
        .args(["-f", "1"]);

    let output = cmd.output().expect("Failed to execute command");
    let expected = "Damage estimation = 22 - 27 (29 - 34)\n";
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert_eq!(actual, expected);
}

#[test]
fn it_outputs_correct_damage_estimation_with_some_args() {
    let mut cmd = get_test_bin("dofus-dmg-calculator");

    cmd
        .args(["-i", "9"])
        .args(["-j", "11"])
        .args(["-k", "12"])
        .args(["-l", "14"])
        .args(["-p", "12"]);

    let output = cmd.output().expect("Failed to execute command");
    let expected = "Damage estimation = 10 - 12 (13 - 15)\n";
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert_eq!(actual, expected);
}
