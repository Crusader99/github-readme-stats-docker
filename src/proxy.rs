use hyper::client::ResponseFuture;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Response, Server, Uri};
use std::env::var;
use std::net::SocketAddr;
use url::Url;

pub async fn start_http_proxy() {
    println!("Starting proxy server to prevent unauthorized access...");

    // Proxy server address and port
    let proxy_address = SocketAddr::from(([0, 0, 0, 0], 80));

    // Call MakeServiceFn that generates the service
    let make_service = make_service_fn(|_connection| async {
        // The service is created from the handle_request function
        Ok::<_, Error>(service_fn(handle_request))
    });

    // Initialize the Hyper server
    let server = Server::bind(&proxy_address).serve(make_service);
    println!("Proxy server listening on {}", proxy_address);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(reqest: Request<Body>) -> Result<Response<Body>, Error> {
    // Target server host address
    let target_host = format!("http://localhost:9000");
    let target_query = reqest
        .uri()
        .path_and_query()
        .expect("Path & query should not be empty");
    let target_path = format!("{}{}", target_host, target_query.as_str());

    // Analyze the HTTP request and decide if it should be approved
    match check_request_approved(target_path) {
        Ok(()) => {
            // Forward the request to the target server
            let forwarded_request = forward_request_to(reqest, target_host);
            forwarded_request.await
        }
        Err(error) => {
            // If the request is not approved, send an appropriate error response to the client
            let message = format!("Forbidden: {}", error);
            let response = Response::builder()
                .status(403)
                .body(Body::from(message))
                .unwrap();
            Ok(response)
        }
    }
}

fn forward_request_to(reqest: Request<Body>, target_path: String) -> ResponseFuture {
    // Target server address
    let target_uri: Uri = target_path.parse().unwrap();

    // Initialize the HTTP client
    let client = Client::new();

    // Create a new request builder and transfer the original request information
    let mut target_req_builder = Request::builder()
        .method(reqest.method().clone())
        .uri(target_uri.clone());

    // Copy the headers of the original request to the target server
    for (header_name, header_value) in reqest.headers() {
        target_req_builder = target_req_builder.header(header_name.clone(), header_value.clone());
    }

    // Create a new request
    let target_req = target_req_builder.body(reqest.into_body()).unwrap();

    // Forward the request to the target server
    return client.request(target_req);
}

// Analyze the HTTP request path and decide if it should be approved
pub fn check_request_approved(target_path: String) -> Result<(), String> {
    let url = Url::parse(&target_path).expect(&format!("Unable to parse path: {}", target_path));
    let query_parameters: Vec<_> = url
        .query_pairs()
        .filter(|(key, _)| key == "username")
        .collect();
    if query_parameters.len() < 1 {
        return Err(String::from("No username provided in request!"));
    } else if query_parameters.len() > 1 {
        return Err(String::from("Multiple usernames provided in request!"));
    }
    let github_user_provided = query_parameters[0].1.to_string();
    let github_user_expected = var("GITHUB_USER").unwrap_or("none".to_string());

    // Compare user from environment variable with the provided user in HTTP query path
    if github_user_provided == github_user_expected {
        return Ok(());
    } else {
        return Err(format!(
            "Username in query '{}' does not match the configured username '{}'!",
            github_user_provided, github_user_expected
        ));
    }
}
