#[path = "../src/proxy.rs"]
mod proxy;

#[test]
fn check_approval() {
    let path = "http://localhost:8080/top-langs?username=none&layout=compact";
    let result = proxy::is_request_approved(path.to_string());
    assert_eq!(result, true);
}

#[test]
fn check_deny() {
    let path = "http://localhost:8080/top-langs?username=another&layout=compact";
    let result = proxy::is_request_approved(path.to_string());
    assert_eq!(result, false);
}


#[test]
#[should_panic]
fn check_multi_user() {
    let path = "http://localhost:8080/top-langs?username=none&username=another2&layout=compact";
    proxy::is_request_approved(path.to_string());
}

#[test]
#[should_panic]
fn check_no_user() {
    let path = "http://localhost:8080/top-langs?layout=compact";
    proxy::is_request_approved(path.to_string());
}
