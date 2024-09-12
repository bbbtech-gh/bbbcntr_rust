use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::controllers::*;
use crate::init_router;
use axum::{http::{Request, Method}, body::Body, routing::{get, post}, Extension, Router};
use tower::util::ServiceExt;


#[cfg(test)]
#[tokio::test]
async fn check_database_connectivity(){
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to make connection");

    // assert to check if connection is successful
    assert_eq!(pool.is_closed(),false);
}

#[cfg(test)]
async fn create_connection_pool() -> PgPool{
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to make connection");

    return pool;
}

#[cfg(test)]
#[tokio::test]
async fn test_user_create(){
    let pool = create_connection_pool().await;
    
    let app = init_router().layer(Extension(pool));

    let req = Request::builder()
        .method(Method::POST)
        .uri("/users")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "email": "john@doe",
                "bbb_id": "john.doe",
                "data": {}
            }"#,
        ))
        .unwrap();

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
}

#[cfg(test)]
#[tokio::test]
async fn test_client_create(){
    let pool = create_connection_pool().await;
    
    let app = init_router().layer(Extension(pool.clone()));

    let req = Request::builder()
        .method(Method::POST)
        .uri("/clients")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "email": "john@doe.pro",
                "domain": "john.doemain",
                "detail": "detail i guess"
            }"#,
        ))
        .unwrap();

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    // println!("{:?}", response);

    assert_eq!(response.status(), 201);

    assert!(approve_client("john@doe.pro".to_string(), "john.doemain".to_string(), pool).await.is_ok());
}

#[cfg(test)]
#[tokio::test]
async fn test_token_create(){
    let pool = create_connection_pool().await;
    
    let app = init_router().layer(Extension(pool.clone()));

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/users")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "email": "john1@doe",
                "bbb_id": "john1.doe",
                "data": {}
            }"#,
        ))
        .unwrap();

    let mut response = app
        .clone()
        .oneshot(req)
        .await
        .unwrap();

    // println!("{:?}", response);

    req = Request::builder()
        .method(Method::POST)
        .uri("/clients")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "email": "john1@doe.pro",
                "domain": "john1.doemain",
                "detail": "detail i guess"
            }"#,
        ))
        .unwrap();

    response = app
        .clone()
        .oneshot(req)
        .await
        .unwrap();

    approve_client("john1@doe.pro".to_string(), "john1.doemain".to_string(), pool).await;

    req = Request::builder()
        .method(Method::POST)
        .uri("/tokens")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "client_id": 1,
                "scopes": {
                    "read": "*"
                },
                "email": "john@doe"
            }"#,
        ))
        .unwrap();

    response = app
        .oneshot(req)
        .await
        .unwrap();

    // println!("{:?}", response);

    assert_eq!(response.status(), 201);
}

#[cfg(test)]
#[tokio::test]
async fn test_get_clients(){
    let pool = create_connection_pool().await;
    let app = init_router().layer(Extension(pool.clone()));

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/clients")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "email": "john2@doe.pro",
                "domain": "john2.doemain",
                "detail": "detail i guess"
            }"#,
        ))
        .unwrap();

    let mut response = app
        .clone()
        .oneshot(req)
        .await
        .unwrap();

    approve_client("john2@doe.pro".to_string(), "john2.doemain".to_string(), pool.clone()).await.unwrap();

    req = Request::builder()
        .method(Method::GET)
        .uri("/clients")
        .header("content-type", "application/json")
        .body(Body::from(""))
        .unwrap();

    response = app
        .oneshot(req)
        .await
        .unwrap();

    // println!("{:?}", response);

    assert_eq!(response.status(), 200);
}
