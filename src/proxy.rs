use hyper::client::ResponseFuture;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Response, Server, Uri};
use std::net::SocketAddr;
use std::env::var;

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
    // Analyze the HTTP request and decide if it should be approved
    // TODO: improve check and move variable to extra class
    let github_user = var("GITHUB_USER").expect("Failed to read the GITHUB_USER environment variable");
    let approved = reqest.uri().query().expect("Expect query not to be empty").contains(&github_user);

    if approved {
        // Target server host address
        let target_host = format!("http://localhost:9000");

        // Forward the request to the target server
        let forwarded_request = forward_request_to(reqest, target_host);

        forwarded_request.await
    } else {
        // If the request is not approved, send an appropriate error response to the client
        let response = Response::builder()
            .status(403)
            .body(Body::from("Forbidden"))
            .unwrap();

        Ok(response)
    }
}

fn forward_request_to(reqest: Request<Body>, target_host: String) -> ResponseFuture {
    // Target server address
    let target_uri = format!("{}{}", target_host, reqest.uri().path());
    let target_uri: Uri = target_uri.parse().unwrap();

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

    // reqest.uri_mut()

    // Forward the request to the target server
    return client.request(target_req);
}
