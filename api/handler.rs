use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use askama::Template;
use hello_rust::hello::HelloTemplate;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
  let hello = HelloTemplate { name: "world" };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(hello.render().unwrap().into())?)
}