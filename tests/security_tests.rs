#[path = "../src/proxy.rs"]
mod proxy;

#[test]
fn check_approval() {
    let path = "http://localhost:8080/top-langs?username=none&layout=compact";
    let actual = proxy::check_request_approved(path.to_string());
    let expected = Ok(());
    assert_eq!(actual, expected);
}

#[test]
fn check_deny() {
    let path = "http://localhost:8080/top-langs?username=another&layout=compact";
    let actual = proxy::check_request_approved(path.to_string());
    let expected = Err(
        "Username in query 'another' does not match the configured username 'none'!".to_string(),
    );
    assert_eq!(actual, expected);
}

#[test]
fn check_multi_user() {
    let path = "http://localhost:8080/top-langs?username=none&username=another2&layout=compact";
    let actual = proxy::check_request_approved(path.to_string());
    let expected = Err("Multiple usernames provided in request!".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn check_no_user() {
    let path = "http://localhost:8080/top-langs?layout=compact";
    let actual = proxy::check_request_approved(path.to_string());
    let expected = Err("No username provided in request!".to_string());
    assert_eq!(actual, expected);
}
