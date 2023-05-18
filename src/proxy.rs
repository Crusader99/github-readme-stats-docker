use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use std::convert::Infallible;
use std::net::SocketAddr;

pub async fn start_http_proxy() {
    println!("Starting proxy server to prevent unauthorized access...");

    // Proxy server address and port
    let proxy_addr = SocketAddr::from(([0, 0, 0, 0], 80));

    // Create a MakeServiceFn that generates the service
    let make_svc = make_service_fn(|_conn| async {
        // The service is created from the handle_request function
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Initialize the Hyper server
    let server = Server::bind(&proxy_addr).serve(make_svc);
    // .map_err(|e| eprintln!("Server error: {}", e));

    println!("Proxy server listening on {}", proxy_addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Analyze the HTTP request and decide if it should be approved
    let approved = req.uri().path() == "/foo";

    if approved {
        // Target server address
        let target_uri = format!("http://localhost:9000{}", req.uri().path());
        let target_uri: Uri = target_uri.parse().unwrap();

        // Initialize the HTTP client
        let client = Client::new();

        // Create a new request builder and transfer the original request information
        let mut target_req_builder = Request::builder()
            .method(req.method().clone())
            .uri(target_uri.clone());

        // Copy the headers of the original request to the target server
        for (header_name, header_value) in req.headers() {
            target_req_builder =
                target_req_builder.header(header_name.clone(), header_value.clone());
        }

        // Create a new request
        let target_req = target_req_builder.body(req.into_body()).unwrap();

        // Forward the request to the target server
        let forwarded_request = client.request(target_req);

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
