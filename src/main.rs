extern crate futures;
extern crate hyper;
extern crate http;
extern crate rust_docker;

use futures::future;
use http::{Request, Response, StatusCode};
use hyper::{Body, Method, Server};
use hyper::rt::Future;
use hyper::service::service_fn;
use rust_docker::api::version::Version;
use rust_docker::DockerClient;

fn is_docker_ok() -> bool {
    let docker = match DockerClient::new("unix:///run/docker.sock") {
        Ok(d) => d,
        Err(_) => {
            println!("docker connect");
            return false
        },
    };

    return match docker.get_version_info() {
        Ok(_) => true,
        Err(e) => {
            println!("docker info: {}", e);
            false
        },
    };
}


fn health(_request: &Request<Body>, response: &mut Response<Body>) {
    if is_docker_ok() {
        *response.status_mut() = StatusCode::OK;
    } else {
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    }
}

type FutureResponse = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
fn router(request: Request<Body>) -> FutureResponse {
    let mut response = Response::default();

    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => {},
        (&Method::GET, "/health") => health(&request, &mut response),
        _ => *response.status_mut() = StatusCode::NOT_FOUND,
    }

    return Box::new(future::ok(response));
}

fn main() {
    let addr = ([0, 0, 0, 0], 8000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(router))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Starting server on: {}", addr);
    hyper::rt::run(server);
}
