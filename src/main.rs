use axum::Extension;
use tower_http::trace::TraceLayer;
use bbbcntr_rust::init_router;
use sqlx::postgres::PgPoolOptions;
use anyhow::Context;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let env = std::fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    let pool = PgPoolOptions::new()
    .max_connections(50)
    .connect(&database_url)
    .await
    .context("could not connect to database_url")
    .unwrap();

    // build our application with a route
    let app = init_router()
          .layer(Extension(pool))
          .layer(TraceLayer::new_for_http());
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}