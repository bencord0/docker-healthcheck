extern crate hyper;
extern crate http;
extern crate rust_docker;

use std::process::exit;
use std::sync::Arc;

use http::{Request, Response, StatusCode};
use hyper::{Body, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use rust_docker::api::version::Version;
use rust_docker::DockerClient;

fn is_docker_ok(docker: &DockerClient) -> bool {
    return match docker.get_version_info() {
        Ok(_) => true,
        Err(e) => {
            println!("docker info: {}", e);
            false
        },
    };
}

fn health(_req: Request<Body>, docker: &DockerClient) -> Response<Body> {
    let mut response = Response::default();

    if is_docker_ok(&docker) {
        *response.status_mut() = StatusCode::OK;
    } else {
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    }

    return response;
}

fn main() {
    let addr = ([0, 0, 0, 0], 8000).into();

    let docker = match DockerClient::new("unix:///run/docker.sock") {
        Ok(d) => Arc::new(d),
        Err(e) => {
            println!("Failed to connect to docker socket");
            exit(1);
        },
    };

    let server = Server::bind(&addr)
        .serve(move || {
            let docker = docker.clone();
            service_fn_ok(move |req| {
                health(req, &docker)
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Starting server on: {}", addr);
    hyper::rt::run(server);
}
