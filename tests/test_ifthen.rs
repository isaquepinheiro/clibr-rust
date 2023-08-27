#[path = "../src/commands/core/clibr_ifthen.rs"]
mod clibr_ifthen;
use crate::clibr_ifthen::IfThen;

#[cfg(test)]
#[test]
fn if_then_with_bool() {
    let if_true: IfThen<bool> = IfThen::new().when(true).then(true).else_or(false);
    let if_false: IfThen<bool> = IfThen::new().when(false).then(true).else_or(false);

    assert_eq!(if_true.evaluate(), true);
    assert_eq!(if_false.evaluate(), false);
}

#[test]
fn if_then_with_float() {
    let if_true: IfThen<f64> = IfThen::new().when(true).then(3.14).else_or(2.71);
    let if_false: IfThen<f64> = IfThen::new().when(false).then(3.14).else_or(2.71);

    assert_eq!(if_true.evaluate(), 3.14);
    assert_eq!(if_false.evaluate(), 2.71);
}

#[test]
fn if_then_with_string() {
    let if_true: IfThen<String> = IfThen::new()
        .when(true)
        .then("True".to_string())
        .else_or("False".to_string());
    let if_false: IfThen<String> = IfThen::new()
        .when(false)
        .then("True".to_string())
        .else_or("False".to_string());

    assert_eq!(if_true.evaluate(), "True");
    assert_eq!(if_false.evaluate(), "False");
}
