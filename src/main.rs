use axum::Extension;
use tower_http::trace::TraceLayer;
use bbbcntr_rust::init_router;
use sqlx::postgres::PgPoolOptions;
use anyhow::Context;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // build our application with a route
    let app = init_router()
          .layer(Extension(pool))
          .layer(TraceLayer::new_for_http());
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}