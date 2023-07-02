use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Handle the incoming request here
    let response_body = "Hello, World!";
    let response = Response::new(Body::from(response_body));

    Ok(response)
}

#[tokio::main]
async fn main() {
    // Create a service from the `handle_request` function
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    // Create a TCP listener bound to localhost on port 3000
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}