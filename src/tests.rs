use crate::lambda::Lambda;

#[test]
#[allow(non_snake_case)]
fn test_true() {
    let T = Lambda::function(vec!["x", "y"], Lambda::variable("x"));
    let result = Lambda::application(T, vec![Lambda::variable("x"), Lambda::variable("y")]);
    assert_eq!(result.reduce(), Lambda::variable("x"));
}

#[test]
#[allow(non_snake_case)]
fn test_false() {
    let F = Lambda::function(vec!["x", "y"], Lambda::variable("y"));
    let result = Lambda::application(F, vec![Lambda::variable("x"), Lambda::variable("y")]);
    assert_eq!(result.reduce(), Lambda::variable("y"));
}

#[test]
fn test_application_order() {
    let expr = Lambda::Application(
        Box::new(Lambda::Application(
            Box::new(Lambda::Variable("a".into())),
            Box::new(Lambda::Variable("b".into())),
        )),
        Box::new(Lambda::Variable("c".into())),
    );

    let expr2 = Lambda::from_string("(a b) c").unwrap();
    assert_eq!(expr, expr2);
}
