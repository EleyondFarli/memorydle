mod api;

#[macro_use]
extern crate rocket;

use sqlx::types::chrono;
use sqlx::{Pool, Sqlite};

// TODO: create multiple modules to better strutcure the project
#[derive(sqlx::FromRow, Debug)]
struct Image {
    image_id: i32,
    puzzle_id: i32,
    image_path: String,
    display_order: i8,
}

#[derive(sqlx::FromRow, Debug)]
struct Country {
    country_id: i32,
    name: String,
}

#[derive(sqlx::FromRow, Debug)]
struct User {
    user_uuid: String,
    username: String,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
struct Puzzle {
    puzzle_id: i32,
    //TODO: Check if NaiveDate actually works here or I need to switch to DateTime
    puzzle_date: chrono::NaiveDate,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn fetch_daily_puzzle(connection: Pool<Sqlite>, date: &str) -> Option<Puzzle> {
    let daily_puzzle: Option<Puzzle> =
        sqlx::query_as("SELECT * FROM puzzles WHERE puzzle_date = $1")
            .bind(date)
            .fetch_optional(&connection)
            .await
            .unwrap();
    return daily_puzzle;
}

async fn connect_to_db() -> Pool<Sqlite> {
    let file_name = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(file_name)
        .create_if_missing(false);

    sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .expect("Failed to connect to database")
}

async fn fetch_images(connection: Pool<Sqlite>) -> Vec<Image> {
    let images: Vec<Image> = sqlx::query_as("SELECT * FROM images")
        .fetch_all(&connection)
        .await
        .unwrap();
    println!("{:?}", images);
    images
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let db_connection = connect_to_db().await;
    // let _images = fetch_images(connection).await;

    rocket::build()
        .manage(db_connection)
        .mount( "/",
        routes![
            api::get_puzzle,
            api::get_user,
            api::create_user,
            api::delete_user,
            api::update_user
        ],
    ).register("/", catchers![api::error_page])
}
