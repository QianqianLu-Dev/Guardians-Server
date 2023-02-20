use axum::{
    Json,
    extract::{Extension, Path},
    routing::{get, post},
    Router,
};

pub fn init_routers() -> Router {
    const v1:str = "/v1";
    let app = Router::new().route(v1+"/firewall"  , get(|| async { "Hello, World!" }))
    .route(v1+"/authuser",get(|| async { "Hello, World!" }))
    .route(v1+"/authteam",get(|| async { "Hello, World!" }))
    .route(v1+"/project",get(|| async { "Hello, World!" }))
    .route(v1+"/module",get(|| async { "Hello, World!" }))
    .route(v1+"/file",get(|| async { "Hello, World!" }))
    .route(v1+"/verify",get(|| async { "Hello, World!" }))
    .route(v1+"/department",get(|| async { "Hello, World!" })) 
    .route(v1+"/hr",get(|| async { "Hello, World!" }))  
    .route(v1+"/email",get(|| async { "Hello, World!" }));
    return app
}