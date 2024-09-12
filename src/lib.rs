pub mod api;
pub mod models;
pub mod controllers;
pub mod errors;
pub mod tests;

use axum::{
    Router, 
    routing::{get, post},
    response::Html,
    Extension
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::controllers::*;

async fn home() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link rel="stylesheet" href="/static/css/index.css" />
            <link href="https://unpkg.com/pattern.css" rel="stylesheet" />
            
          </head>
          <body class="bg-black h-screen w-screen ">
            <div class="w-screen">
              <div class="h-screen w-full overflow-y-scroll no-scrollbar">
                <main class="text-gray-800 font-bold text-9xl text-center pattern-cross-dots-xl h-[60vh]  bg-gray-950 ">
                  <h1 class="
                    bg-clip-text text-transparent
                    bg-gradient-to-r from-red-600
                    to-purple-500 pt-20 py-10 animate-glotext no-scrollbar">
                    BBB <b class="font-black">Cntr</b>
                  </h1>
                  <h3 class="text-white text-xl font-medium pb-10">
                    All your data.
                    In one place.
                    Freedom.
                  </h3>
                  <h1 class="text-3xl bottom-0 text-white">
                    Get Started
                  </h1>
                </main>
                <div class="w-screen h-screen bg-gradient-to-r from-black via-purple-400 to-black pt-2 text-center">
                  <div class="p-4 font-mono text-2xl font-black text-white bg-gradient-to-b from-gray-950 to-transparent h-full">
                    Sigma sigma on the wall
                  </div>
                </div>
              </div>
            </div>
          </body>
        </html>
        "#
    )
}

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/tokens", get(all_tokens))
        .route("/tokens/:id", get(get_token_by_id))
        .route("/clients", get(all_clients))
        .route("/clients/:id", get(get_client_by_id))
        .route("/users", get(all_users))
        .route("/tokens", post(new_token))
        .route("/clients", post(new_client))
        .route("/users", post(new_user))
        .nest_service("/static", ServeDir::new("static"))
}