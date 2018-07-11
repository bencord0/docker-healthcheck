extern crate hyper;
extern crate http;
extern crate rust_docker;

use http::{Request, Response, StatusCode};
use hyper::{Body, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
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

fn health(_req: Request<Body>) -> Response<Body> {
    let mut response = Response::default();

    if is_docker_ok() {
        *response.status_mut() = StatusCode::OK;
    } else {
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    }

    return response;
}

fn main() {
    let addr = ([0, 0, 0, 0], 8000).into();

    let svc = || {
        service_fn_ok(health)
    };

    let server = Server::bind(&addr)
        .serve(svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Starting server on: {}", addr);
    hyper::rt::run(server);
}
