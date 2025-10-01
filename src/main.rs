use crate::lambda::Lambda;

mod lambda;

fn main() {
    let T = Lambda::function(vec!["x", "y"], Lambda::variable("x"));
    let F = Lambda::function(vec!["x", "y"], Lambda::variable("y"));
    let result = Lambda::application(F, vec![Lambda::variable("x"), Lambda::variable("y")]);
    println!("{:?}", result.reduce());
}

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
