extern crate docker;
extern crate hyper;
extern crate http;

use docker::Docker;
use http::{Request, Response, StatusCode};
use hyper::{Body, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn is_docker_ok() -> bool {
    let mut docker = match Docker::connect("unix:///run/docker.sock") {
        Ok(d) => d,
        Err(_) => {
            println!("docker connect");
            return false
        },
    };

    return match docker.get_system_info() {
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
