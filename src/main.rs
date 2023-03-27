use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use mongodb::{Client, options::{ClientOptions}};
use mongodb::bson::doc;
use once_cell::sync::Lazy;
use std::env;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::error::Error;
use tracing::{self};
use tracing_subscriber::{self};

//TODO - CI/CD for gitlab
//TODO - Readme: howto

// const MONGO_URI: &str = "mongodb://localhost";
static API_EP: Lazy<String> = Lazy::new(|| env::var("API_ENDPOINT")
    .expect("You must set the API_ENDPOINT environment var!"));
static MONGO_URI: Lazy<String> = Lazy::new(|| env::var("MONGODB_URI")
    .expect("You must set the MONGODB_URI environment var!"));

#[tokio::main]
async fn main() {

    if MONGO_URI.is_empty() {
       panic!()
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `health_check`
        .route("/health-check", get(health_check))
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));
        .route(API_EP.as_str(), post(create_user));


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn health_check() -> (StatusCode, &'static str) {
    tracing::info!("/health-check called");
    (StatusCode::OK, "Health, Check!")
}


async fn mongo_connect(username: &String, dbname: &String) -> Result<(), Box<dyn Error>> {
    let options = ClientOptions::parse(MONGO_URI.as_str()).await?;
    let client = Client::with_options(options)?;

    let user: Vec<&str>  = username.split("@").collect();
    let record = doc!{"createUser": user[0], "pwd": "Test123", "roles": [{ "role": "readWrite",
        "db": dbname }] };
    let _insert_result = client
        .database(dbname)
        .run_command(record , None)
        .await?;

    Ok(())
}

async fn create_user(Json(payload): Json<FromAngular>) -> (StatusCode, Json<User>) {
    let user = User {
        first_name: payload.firstName,
        last_name: payload.lastName,
        org_name: payload.orgName,
        db_name: payload.dbName,
        email_address: payload.email,
    };

    // mongo_connect(&user.email_address, &user.db_name ).await.expect("panic message");
    match mongo_connect(&user.email_address, &user.db_name ).await {
        Ok(()) => tracing::info!("Status Code: {:?}\t User: {:?}", StatusCode::CREATED, Json(&user)),
        Err(e) => tracing::warn!(e)
    }

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct FromAngular {
    firstName: String,
    lastName: String,
    orgName: String,
    dbName: String,
    email: String,
}

// the output to our `create_user` handler
#[derive(Debug)]
#[derive(Serialize)]
struct User {
    first_name: String,
    last_name: String,
    org_name: String,
    db_name: String,
    email_address: String,
}
