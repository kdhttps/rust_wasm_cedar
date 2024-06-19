use cedar_policy::*;

fn main() {
    const POLICY_SRC: &str = r#"
    permit(principal == User::"alice", action == Action::"view", resource == File::"93");
    "#;
    let policy: PolicySet = POLICY_SRC.parse().unwrap();

    let action = r#"Action::"view""#.parse().unwrap();
    let alice = r#"User::"alice""#.parse().unwrap();
    let file = r#"File::"93""#.parse().unwrap();
    let request = Request::new(
        Some(alice),
        Some(action),
        Some(file),
        Context::empty(),
        None,
    )
    .unwrap();

    let entities = Entities::empty();
    let authorizer = Authorizer::new();
    let answer = authorizer.is_authorized(&request, &policy, &entities);
    println!("{:?}", answer.decision())
}
