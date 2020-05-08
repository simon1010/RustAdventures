extern crate hyper;
use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::*;

fn main() {
  let addr = ([127,0,0,1], 8080).into();
  let builder = Server::bind(&addr);

  let server = builder.serve(||{
    service::service_fn_ok(|_|{
      Response::new(Body::from("Almost microservices..."))
    })
  });

  let server = server.map_err(drop);

  hyper::rt::run(server);
}
