use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Extension, Path},

};



#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = init_routers();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



pub fn init_routers() -> Router {
    let app = Router::new().route("/v1/firewall", get(|| async { "Hello, World!" }))
    .route("/v1/authuser",get(|| async { "Hello, World!" }))
    .route("/v1/authteam",get(|| async { "Hello, World!" }))
    .route("/v1/project",get(|| async { "Hello, World!" }))
    .route( "/v1/module",get(|| async { "Hello, World!" }))
    .route("/v1/file",get(|| async { "Hello, World!" }))
    .route("/v1/verify",get(|| async { "Hello, World!" }))
    .route( "/v1/department",get(|| async { "Hello, World!" })) 
    .route( "/v1/hr",get(|| async { "Hello, World!" }))  
    .route("/v1/email",post(|| async { "Hello, World!" }));
    return app
}