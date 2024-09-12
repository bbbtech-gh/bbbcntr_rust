use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::Path;

use axum::{Extension, Json};
use sqlx::PgPool;

use crate::errors::CustomError;
use crate::models::*;


macro_rules! check_unique {
    (
        $value:expr, 
        $column:ident, 
        $table:ident, 
        $error:expr,
        $pool:expr
    ) => {
        let sql = format!("SELECT COUNT(*) FROM {} where {}=$1", stringify!($table), stringify!($column)).to_string();

        let exists = sqlx::query_scalar::<_, i64>(&sql)
            .bind($value)
            .fetch_one($pool)
            .await
            .map_err(|e| {
                println!("{:?}", e);
                $error
            })? > 0;
        println!("{:?}", exists);
        if exists {
            return Err($error)
        }
        // println!("Hello!")
    };
}

pub async fn approve_client(email: String, domain: String, pool: PgPool) -> Result<(), CustomError> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&durl)
    //     .await
    //     .expect("unable to make connection");

    let mut sql = "SELECT * FROM pending_clients where email=$1 and domain=$2".to_string();

    let client = sqlx::query_as::<_, NewClient>(&sql)
        .bind(email)
        .bind(domain)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::ClientNotFound
        })?;
        // .unwrap();

    // here we know the client exists, so we move it from pending_clients to clients.
    sql = "INSERT INTO clients (email, domain, detail, created_on, updated_on) values ($1, $2, $3, $4, $5)".to_string();

    sqlx::query(&sql)
        .bind(&client.email)
        .bind(&client.domain)
        .bind(&client.detail)
        .bind(chrono::offset::Local::now())
        .bind(chrono::offset::Local::now())
        .execute(&pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            CustomError::InternalServerError
        })?;
    return Ok(())
}


pub async fn all_tokens(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM tokens ".to_string();

    let token = sqlx::query_as::<_, Token>(&sql).fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(token))
}

pub async fn get_token_by_id(
    Path(id):Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result <Json<Token>, CustomError> {

    let sql = "SELECT * FROM tokens where id=$1".to_string();

    let task: Token = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::TokenNotFound
        })?;


    Ok(Json(task))  
}


pub async fn all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM users".to_string();

    let task = sqlx::query_as::<_, User>(&sql).fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_user_by_id(
    Path(id):Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result <Json<User>, CustomError> {

    let sql = "SELECT * FROM users where id=$1".to_string();

    let task: User = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::UserNotFound
        })?;


    Ok(Json(task))  
}


pub async fn all_clients(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM clients".to_string();

    let task = sqlx::query_as::<_, Client>(&sql).fetch_all(&pool).await.unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_client_by_id(
    Path(id):Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result <Json<Client>, CustomError> {

    let sql = "SELECT * FROM clients where id=$1".to_string();

    let task: Client = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::ClientNotFound
        })?;


    Ok(Json(task))  
}


// POST
#[axum::debug_handler]
pub async fn new_token(
    Extension(pool): Extension<PgPool>,
    Json(token): Json<NewToken>, // JSON at end to avoid
    // error: `Json<_>` consumes the request body and thus must be the last argument to the handler function
) -> Result <(StatusCode, Json<NewToken>), CustomError> {

    println!("{:?} {:?}", token, chrono::offset::Local::now());
    
    let sql = "INSERT INTO tokens (client_id, scopes, email, created_on, updated_on) values ($1, $2, $3, $4, $5)";

    let _ = sqlx::query(&sql)
        .bind(&token.client_id)
        .bind(&token.scopes)
        .bind(&token.email)
        .bind(chrono::offset::Local::now())
        .bind(chrono::offset::Local::now())
        .execute(&pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(token)))
}

#[axum::debug_handler]
pub async fn new_client(
    Extension(pool): Extension<PgPool>,
    Json(client): Json<NewClient>, // JSON at end to avoid
    // error: `Json<_>` consumes the request body and thus must be the last argument to the handler function
) -> Result <(StatusCode, Json<NewClient>), CustomError> {

    check_unique!( 
        &client.domain,
        domain,
        pending_clients,
        CustomError::ClientApplicationExists,
        &pool
    );

    check_unique!( 
        &client.domain,
        domain,
        clients,
        CustomError::ClientExists,
        &pool
    );

    check_unique!( 
        &client.email,
        email,
        pending_clients,
        CustomError::ClientApplicationExists,
        &pool
    );

    check_unique!( 
        &client.email,
        email,
        clients,
        CustomError::ClientExists,
        &pool
    );

    // println!("{:?} {:?}", client, chrono::offset::Local::now());
    
    let sql = "INSERT INTO pending_clients (email, domain, detail, created_on) values ($1, $2, $3, $4)";

    let _ = sqlx::query(&sql)
        .bind(&client.email)
        .bind(&client.domain)
        .bind(&client.detail)
        .bind(chrono::offset::Local::now())
        .execute(&pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(client)))
}

#[axum::debug_handler]
pub async fn new_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<NewUser>, // JSON at end to avoid
    // error: `Json<_>` consumes the request body and thus must be the last argument to the handler function
) -> Result <(StatusCode, Json<NewUser>), CustomError> {

    check_unique!( 
        &user.email,
        email,
        users,
        CustomError::UserExists,
        &pool
    );

    check_unique!( 
        &user.bbb_id,
        bbb_id,
        users,
        CustomError::UserExists,
        &pool
    );

    // println!("{:?} {:?}", client, chrono::offset::Local::now());
    
    let sql = "INSERT INTO users (bbb_id, email, phone, data, created_on, updated_on) values ($1, $2, $3, $4, $5, $6)";

    let _ = sqlx::query(&sql)
        .bind(&user.bbb_id)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.data)
        .bind(chrono::offset::Local::now())
        .bind(chrono::offset::Local::now())
        .execute(&pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}
